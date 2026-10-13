use std::sync::atomic::{AtomicU8, Ordering};

/// 托盘菜单跨线程指令编码
pub const TRAY_CMD_NONE: u8 = 0;
pub const TRAY_CMD_BACKUP: u8 = 1;
pub const TRAY_CMD_ABOUT: u8 = 2;
pub const TRAY_CMD_QUIT: u8 = 3;

static PENDING_TRAY_CMD: AtomicU8 = AtomicU8::new(TRAY_CMD_NONE);

/// 读取并消费待处理的系统托盘指令
pub fn take_tray_command() -> u8 {
    PENDING_TRAY_CMD.swap(TRAY_CMD_NONE, Ordering::SeqCst)
}

#[cfg(target_os = "windows")]
mod win_tray {
    use super::*;
    use std::mem::size_of;
    use windows::{
        core::{w, PCWSTR},
        Win32::{
            Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM},
            Graphics::Gdi::{
                CreateBitmap, CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject,
                BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
            },
            System::LibraryLoader::GetModuleHandleW,
            UI::{
                Shell::{
                    Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
                    NOTIFYICONDATAW,
                },
                WindowsAndMessaging::{
                    AppendMenuW, CreateIconIndirect, CreatePopupMenu, CreateWindowExW,
                    DefWindowProcW, DestroyMenu, DispatchMessageW, FindWindowW, GetCursorPos,
                    GetMessageW, IsIconic, PostMessageW, PostQuitMessage, RegisterClassW,
                    SendMessageW, SetForegroundWindow, ShowWindow, TrackPopupMenu,
                    TranslateMessage, CW_USEDEFAULT, HICON, ICONINFO, ICON_BIG, ICON_SMALL,
                    MF_SEPARATOR, MF_STRING, MSG, SW_RESTORE, SW_SHOW, TPM_BOTTOMALIGN,
                    TPM_LEFTALIGN, TPM_RETURNCMD, TPM_RIGHTBUTTON, WINDOW_EX_STYLE, WM_APP,
                    WM_CLOSE, WM_DESTROY, WM_LBUTTONDBLCLK, WM_LBUTTONUP, WM_NULL, WM_RBUTTONUP,
                    WM_SETICON, WNDCLASSW, WS_OVERLAPPEDWINDOW,
                },
            },
        },
    };

    const WM_TRAYICON: u32 = WM_APP + 101;
    const IDM_OPEN: usize = 1001;
    const IDM_BACKUP: usize = 1002;
    const IDM_ABOUT: usize = 1003;
    const IDM_EXIT: usize = 1004;

    /// 使用纯 Rust 像素级光栅化生成 32x32 抗锯齿 NovaOps Pro 品牌图标 (HICON)
    pub fn create_novaops_hicon() -> Option<HICON> {
        const W: i32 = 32;
        const H: i32 = 32;

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: W,
                biHeight: -H, // top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        unsafe {
            let hdc = CreateCompatibleDC(None);
            let mut bits_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
            let hbm_color =
                CreateDIBSection(Some(hdc), &bmi, DIB_RGB_COLORS, &mut bits_ptr, None, 0).ok()?;
            let _ = DeleteDC(hdc);

            if bits_ptr.is_null() {
                let _ = DeleteObject(hbm_color.into());
                return None;
            }

            let pixels = std::slice::from_raw_parts_mut(bits_ptr as *mut u32, (W * H) as usize);

            for y in 0..H {
                for x in 0..W {
                    let fx = x as f32 + 0.5;
                    let fy = y as f32 + 0.5;
                    let cx = 16.0_f32;
                    let cy = 16.0_f32;

                    // 圆角矩形底座 SDF (中心 16,16, 半宽高 14, 圆角半径 6.5)
                    let qx = (fx - cx).abs() - (14.0 - 6.5);
                    let qy = (fy - cy).abs() - (14.0 - 6.5);
                    let dist_round_box = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt()
                        + qx.max(qy).min(0.0)
                        - 6.5;

                    if dist_round_box > 0.6 {
                        pixels[(y * W + x) as usize] = 0x00000000;
                        continue;
                    }

                    // 边缘平滑抗锯齿 alpha
                    let alpha = (0.6 - dist_round_box).clamp(0.0, 1.0);

                    // 背景渐变色 (#2563EB -> #0EA5E9)
                    let t = ((fx + fy) / 64.0).clamp(0.0, 1.0);
                    let mut r = 37.0 * (1.0 - t) + 14.0 * t;
                    let mut g = 99.0 * (1.0 - t) + 165.0 * t;
                    let mut b = 235.0 * (1.0 - t) + 233.0 * t;

                    // 六边形星核与中心几何图案
                    let dx = (fx - cx).abs();
                    let dy = (fy - cy).abs();
                    let hex_dist = (dx * 0.866025 + dy * 0.5).max(dy);

                    // 外层六边形亮环 (半径 8.8 ~ 10.4)
                    let on_outer_ring = (hex_dist - 9.5).abs() < 1.05;
                    // 内层实心六棱星核 (半径 <= 5.2，且中心留出深蓝核心点 r <= 2.1)
                    let center_r = (dx * dx + dy * dy).sqrt();
                    let on_inner_core = hex_dist <= 5.2 && center_r >= 2.1;

                    if on_outer_ring || on_inner_core {
                        r = 250.0;
                        g = 253.0;
                        b = 255.0;
                    }

                    let a_u8 = (alpha * 255.0) as u32;
                    let r_u8 = (r * alpha) as u32;
                    let g_u8 = (g * alpha) as u32;
                    let b_u8 = (b * alpha) as u32;
                    pixels[(y * W + x) as usize] =
                        (a_u8 << 24) | (r_u8 << 16) | (g_u8 << 8) | b_u8;
                }
            }

            let mask_bytes = [0u8; ((W * H) / 8) as usize];
            let hbm_mask = CreateBitmap(W, H, 1, 1, Some(mask_bytes.as_ptr() as *const _));

            let icon_info = ICONINFO {
                fIcon: true.into(),
                xHotspot: 0,
                yHotspot: 0,
                hbmMask: hbm_mask,
                hbmColor: hbm_color,
            };

            let hicon = CreateIconIndirect(&icon_info).ok();
            let _ = DeleteObject(hbm_color.into());
            let _ = DeleteObject(hbm_mask.into());
            hicon
        }
    }

    /// 查找当前进程的 GPUI 主窗口并唤醒/设置图标
    pub fn find_and_activate_main_window(hicon: Option<HICON>, restore: bool) {
        unsafe {
            let hwnd = FindWindowW(w!("Zed::Window"), PCWSTR::null()).unwrap_or_default();
            if !hwnd.0.is_null() {
                if let Some(icon) = hicon {
                    let _ = SendMessageW(
                        hwnd,
                        WM_SETICON,
                        Some(WPARAM(ICON_BIG as usize)),
                        Some(LPARAM(icon.0 as isize)),
                    );
                    let _ = SendMessageW(
                        hwnd,
                        WM_SETICON,
                        Some(WPARAM(ICON_SMALL as usize)),
                        Some(LPARAM(icon.0 as isize)),
                    );
                }
                if restore {
                    if IsIconic(hwnd).as_bool() {
                        let _ = ShowWindow(hwnd, SW_RESTORE);
                    } else {
                        let _ = ShowWindow(hwnd, SW_SHOW);
                    }
                    let _ = SetForegroundWindow(hwnd);
                }
            }
        }
    }

    unsafe extern "system" fn tray_wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match msg {
                WM_TRAYICON => {
                    let event = (lparam.0 as u32) & 0xFFFF;
                    if event == WM_LBUTTONUP || event == WM_LBUTTONDBLCLK {
                        find_and_activate_main_window(None, true);
                    } else if event == WM_RBUTTONUP {
                        let mut pt = POINT::default();
                        let _ = GetCursorPos(&mut pt);
                        if let Ok(hmenu) = CreatePopupMenu() {
                            let _ = AppendMenuW(hmenu, MF_STRING, IDM_OPEN, w!("打开 NovaOps Pro 控制台"));
                            let _ = AppendMenuW(hmenu, MF_STRING, IDM_BACKUP, w!("立即执行数据热备份"));
                            let _ = AppendMenuW(hmenu, MF_STRING, IDM_ABOUT, w!("关于 NovaOps Pro"));
                            let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR::null());
                            let _ = AppendMenuW(hmenu, MF_STRING, IDM_EXIT, w!("安全退出程序"));

                            let _ = SetForegroundWindow(hwnd);
                            let cmd = TrackPopupMenu(
                                hmenu,
                                TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON | TPM_RETURNCMD,
                                pt.x,
                                pt.y,
                                Some(0),
                                hwnd,
                                None,
                            );
                            let _ = PostMessageW(Some(hwnd), WM_NULL, WPARAM(0), LPARAM(0));
                            let _ = DestroyMenu(hmenu);

                            match cmd.0 as usize {
                                IDM_OPEN => {
                                    find_and_activate_main_window(None, true);
                                }
                                IDM_BACKUP => {
                                    find_and_activate_main_window(None, true);
                                    PENDING_TRAY_CMD.store(TRAY_CMD_BACKUP, Ordering::SeqCst);
                                }
                                IDM_ABOUT => {
                                    find_and_activate_main_window(None, true);
                                    PENDING_TRAY_CMD.store(TRAY_CMD_ABOUT, Ordering::SeqCst);
                                }
                                IDM_EXIT => {
                                    PENDING_TRAY_CMD.store(TRAY_CMD_QUIT, Ordering::SeqCst);
                                    let main_hwnd =
                                        FindWindowW(w!("Zed::Window"), PCWSTR::null()).unwrap_or_default();
                                    if !main_hwnd.0.is_null() {
                                        let _ = PostMessageW(
                                            Some(main_hwnd),
                                            WM_CLOSE,
                                            WPARAM(0),
                                            LPARAM(0),
                                        );
                                    }
                                    let _ = PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0));
                                }
                                _ => {}
                            }
                        }
                    }
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }

    pub fn spawn_tray_thread() {
        std::thread::Builder::new()
            .name("novaops-system-tray".into())
            .spawn(|| unsafe {
                let Some(hicon) = create_novaops_hicon() else {
                    return;
                };

                // 延迟 180ms 等待主窗口创建完成后，立即为主窗口标题栏与任务栏注入应用图标
                std::thread::sleep(std::time::Duration::from_millis(180));
                find_and_activate_main_window(Some(hicon), false);

                let hinstance = GetModuleHandleW(None).unwrap_or_default();
                let class_name = w!("NovaOpsTrayHiddenWindowClass");

                let wc = WNDCLASSW {
                    lpfnWndProc: Some(tray_wnd_proc),
                    hInstance: hinstance.into(),
                    lpszClassName: class_name,
                    ..Default::default()
                };
                let _ = RegisterClassW(&wc);

                let Ok(hwnd) = CreateWindowExW(
                    WINDOW_EX_STYLE(0),
                    class_name,
                    w!("NovaOpsTrayWatcher"),
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    None,
                    None,
                    Some(hinstance.into()),
                    None,
                ) else {
                    return;
                };

                let mut nid = NOTIFYICONDATAW {
                    cbSize: size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: hwnd,
                    uID: 1,
                    uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
                    uCallbackMessage: WM_TRAYICON,
                    hIcon: hicon,
                    ..Default::default()
                };

                let tip = "NovaOps Pro - 企业级云原生管理平台 (运行中)";
                for (i, ch) in tip.encode_utf16().take(nid.szTip.len() - 1).enumerate() {
                    nid.szTip[i] = ch;
                }

                let _ = Shell_NotifyIconW(NIM_ADD, &nid);

                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).into() {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
            })
            .ok();
    }
}

/// 启动系统托盘图标与主窗口自定义应用图标注入
pub fn init_system_tray() {
    #[cfg(target_os = "windows")]
    win_tray::spawn_tray_thread();
}
