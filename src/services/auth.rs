use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub avatar_abbr: String,
    pub is_offline: bool,
}

pub struct AuthService;

impl AuthService {
    /// 使用 Argon2id 对密码进行加盐哈希
    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| format!("Argon2 哈希生成失败: {e}"))
    }

    /// 校验明文密码与 Argon2id 哈希
    pub fn verify_password(password: &str, password_hash: &str) -> bool {
        let parsed_hash = match PasswordHash::new(password_hash) {
            Ok(h) => h,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    /// 初始化本地管理员凭据表，并播种默认离线账户
    pub fn init_auth_tables(conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS admin_credentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                display_name TEXT NOT NULL,
                role TEXT NOT NULL,
                avatar_abbr TEXT NOT NULL,
                last_login TEXT NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM admin_credentials",
            [],
            |r| r.get(0),
        )?;

        if count == 0 {
            // 预设默认离线超级管理员: admin / admin123
            let default_pass_hash = Self::hash_password("admin123")
                .unwrap_or_else(|_| "$argon2id$v=19$m=19456,t=2,p=1$placeholder".into());
            
            conn.execute(
                "INSERT INTO admin_credentials (username, password_hash, display_name, role, avatar_abbr, last_login)
                 VALUES (?1, ?2, ?3, ?4, ?5, datetime('now', 'localtime'))",
                params![
                    "admin",
                    default_pass_hash,
                    "系统管理员 (Admin)",
                    "超级管理员",
                    "SA"
                ],
            )?;

            // 预设辅管理员: ops / ops123
            let ops_pass_hash = Self::hash_password("ops123")
                .unwrap_or_else(|_| "$argon2id$v=19$m=19456,t=2,p=1$placeholder".into());
            
            conn.execute(
                "INSERT INTO admin_credentials (username, password_hash, display_name, role, avatar_abbr, last_login)
                 VALUES (?1, ?2, ?3, ?4, ?5, datetime('now', 'localtime'))",
                params![
                    "ops",
                    ops_pass_hash,
                    "运维值班员 (Ops)",
                    "架构与运维",
                    "OP"
                ],
            )?;
        }

        Ok(())
    }

    /// 离线登录验证
    pub fn authenticate(
        conn: &Connection,
        username: &str,
        password: &str,
    ) -> Result<AuthUser, String> {
        let trimmed_username = username.trim();
        if trimmed_username.is_empty() {
            return Err("请输入登录用户名".into());
        }
        if password.is_empty() {
            return Err("请输入登录密码".into());
        }

        let mut stmt = conn
            .prepare(
                "SELECT id, username, password_hash, display_name, role, avatar_abbr 
                 FROM admin_credentials 
                 WHERE username = ?1 COLLATE NOCASE",
            )
            .map_err(|e| format!("数据库查询异常: {e}"))?;

        let user_record = stmt
            .query_row(params![trimmed_username], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                ))
            });

        match user_record {
            Ok((id, uname, hash, display_name, role, avatar_abbr)) => {
                if Self::verify_password(password, &hash) {
                    // 更新最后登录时间
                    let _ = conn.execute(
                        "UPDATE admin_credentials SET last_login = datetime('now', 'localtime') WHERE id = ?1",
                        params![id],
                    );
                    Ok(AuthUser {
                        id,
                        username: uname,
                        display_name,
                        role,
                        avatar_abbr,
                        is_offline: true,
                    })
                } else {
                    Err("密码校验错误，请检查输入".into())
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                Err(format!("用户 '{trimmed_username}' 不存在"))
            }
            Err(e) => Err(format!("认证失败: {e}")),
        }
    }
}
