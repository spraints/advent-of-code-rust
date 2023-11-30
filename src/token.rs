use std::path::PathBuf;

pub fn set_token(token: String) -> anyhow::Result<()> {
    let cfg_file = token_path()?;
    let cfg_dir = cfg_file
        .parent()
        .ok_or_else(|| anyhow::anyhow!("expected {cfg_file:?} to be in a directory"))?;
    std::fs::create_dir_all(&cfg_dir)?;
    std::fs::write(&cfg_file, token)
        .or_else(|e| anyhow::bail!("error writing token to {cfg_file:?}: {e:?}"))?;
    println!("wrote token to {:?}", cfg_file);
    Ok(())
}

pub fn get_token() -> anyhow::Result<String> {
    let cfg_file = token_path()?;
    std::fs::read_to_string(&cfg_file)
        .or_else(|e| anyhow::bail!("error reading token from {cfg_file:?}: {e:?}"))
}

fn token_path() -> anyhow::Result<PathBuf> {
    let app_dirs = platform_dirs::AppDirs::new(Some("advent-of-code"), false)
        .ok_or_else(|| anyhow::anyhow!("could not find advent-of-code app settings"))?;
    let mut cfg_file = app_dirs.config_dir;
    cfg_file.push("token");
    Ok(cfg_file)
}
