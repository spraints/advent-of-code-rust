use std::path::PathBuf;

pub fn set_token(token: String) -> anyhow::Result<()> {
    let cfg_file = token_path()?;
    std::fs::write(&cfg_file, token)?;
    println!("wrote token to {:?}", cfg_file);
    Ok(())
}

pub fn get_token() -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(token_path()?)?)
}

fn token_path() -> anyhow::Result<PathBuf> {
    let app_dirs = platform_dirs::AppDirs::new(Some("advent-of-code"), false).unwrap();
    let mut cfg_file = app_dirs.config_dir;
    cfg_file.push("token");
    Ok(cfg_file)
}
