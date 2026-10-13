use std::fs;
use std::path::Path;

/// 生成标准 64x64 32-bit BGRA Windows .ico 图标文件（深海科技蓝圆角底座 + 纯白六边形星核立方体）
fn generate_novaops_ico(ico_path: &Path) -> std::io::Result<()> {
    if let Some(parent) = ico_path.parent() {
        fs::create_dir_all(parent)?;
    }

    const W: u32 = 64;
    const H: u32 = 64;
    let mut bgra = vec![0u8; (W * H * 4) as usize];

    // BMP XOR 图像数据是 bottom-up (从底行到顶行)
    for row in 0..H {
        let y = (H - 1 - row) as f32 + 0.5;
        for col in 0..W {
            let x = col as f32 + 0.5;
            let cx = 32.0_f32;
            let cy = 32.0_f32;

            // 圆角矩形底座 SDF (中心 32,32, 半宽高 28, 圆角半径 13)
            let qx = (x - cx).abs() - (28.0 - 13.0);
            let qy = (y - cy).abs() - (28.0 - 13.0);
            let dist_round_box =
                (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt() + qx.max(qy).min(0.0) - 13.0;

            let idx = ((row * W + col) * 4) as usize;
            if dist_round_box > 0.8 {
                bgra[idx] = 0;
                bgra[idx + 1] = 0;
                bgra[idx + 2] = 0;
                bgra[idx + 3] = 0;
                continue;
            }

            let alpha = (0.8 - dist_round_box).clamp(0.0, 1.0);

            // 科技蓝渐变背景 (#2563EB -> #0EA5E9)
            let t = ((x + y) / 128.0).clamp(0.0, 1.0);
            let mut r = 37.0 * (1.0 - t) + 14.0 * t;
            let mut g = 99.0 * (1.0 - t) + 165.0 * t;
            let mut b = 235.0 * (1.0 - t) + 233.0 * t;

            // 六边形星核与拓扑结构
            let dx = (x - cx).abs();
            let dy = (y - cy).abs();
            let hex_dist = (dx * 0.866025 + dy * 0.5).max(dy);
            let center_r = (dx * dx + dy * dy).sqrt();

            // 外层六边形科技星环 (半径 19.0, 线宽 ~2.2)
            let on_outer_ring = (hex_dist - 19.0).abs() < 1.35;
            // 内层实心六棱能量块 (半径 <= 10.6 且留出中心圆核 r >= 4.2)
            let on_inner_core = hex_dist <= 10.6 && center_r >= 4.2;
            // 垂直拓扑连接线
            let on_vertical_spoke = dx < 1.2 && dy >= 10.0 && dy <= 19.0;

            if on_outer_ring || on_inner_core || on_vertical_spoke {
                r = 250.0;
                g = 253.0;
                b = 255.0;
            }

            bgra[idx] = b as u8;
            bgra[idx + 1] = g as u8;
            bgra[idx + 2] = r as u8;
            bgra[idx + 3] = (alpha * 255.0) as u8;
        }
    }

    // AND 掩码 (1 bit per pixel, 每行按 32-bit 对齐: 64 bits = 8 bytes/row)
    let and_mask = vec![0u8; (8 * H) as usize];

    let bmp_header_size = 40u32;
    let image_bytes_size = bmp_header_size + (bgra.len() as u32) + (and_mask.len() as u32);

    let mut ico = Vec::with_capacity((22 + image_bytes_size) as usize);

    // ICONDIR (6 bytes)
    ico.extend_from_slice(&0u16.to_le_bytes()); // Reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // Type: 1 = Icon
    ico.extend_from_slice(&1u16.to_le_bytes()); // Count: 1 image

    // ICONDIRENTRY (16 bytes)
    ico.push(W as u8); // Width
    ico.push(H as u8); // Height
    ico.push(0); // ColorCount
    ico.push(0); // Reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // Planes
    ico.extend_from_slice(&32u16.to_le_bytes()); // BitCount
    ico.extend_from_slice(&image_bytes_size.to_le_bytes()); // BytesInRes
    ico.extend_from_slice(&22u32.to_le_bytes()); // ImageOffset

    // BITMAPINFOHEADER (40 bytes)
    ico.extend_from_slice(&40u32.to_le_bytes()); // biSize
    ico.extend_from_slice(&(W as i32).to_le_bytes()); // biWidth
    ico.extend_from_slice(&((H * 2) as i32).to_le_bytes()); // biHeight (XOR + AND)
    ico.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
    ico.extend_from_slice(&32u16.to_le_bytes()); // biBitCount
    ico.extend_from_slice(&0u32.to_le_bytes()); // biCompression (BI_RGB)
    ico.extend_from_slice(&(bgra.len() as u32).to_le_bytes()); // biSizeImage
    ico.extend_from_slice(&0i32.to_le_bytes()); // biXPelsPerMeter
    ico.extend_from_slice(&0i32.to_le_bytes()); // biYPelsPerMeter
    ico.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
    ico.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

    ico.extend_from_slice(&bgra);
    ico.extend_from_slice(&and_mask);

    fs::write(ico_path, ico)
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        let ico_path = Path::new("assets/app_icon.ico");
        let _ = generate_novaops_ico(ico_path);
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=assets/app_icon.svg");

        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/app_icon.ico");
        let _ = res.compile();
    }
}
