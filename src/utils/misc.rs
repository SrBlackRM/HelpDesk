use md5;

pub fn password_md5_hasher(password: &str) -> String {
    let password_hash = md5::compute(password.as_bytes());
    format!("{:x}", password_hash)
}