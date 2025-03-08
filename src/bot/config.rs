pub fn init_env() -> std::io::Result<()> {
    let env_file: String = std::fs::read_to_string(".env")?;

    for line in env_file.lines() {
        if let Some((key, val)) = line.split_once('=') {
            std::env::set_var(key, val);
        }
    }

    Ok(())
}