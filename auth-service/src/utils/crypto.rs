pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    // FIXME: Replace with real hashing logic
    let hashed = format!("hashed_{}", password);
    Ok(hashed)
}
