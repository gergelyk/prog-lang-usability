use anyhow::{Result, Context, bail};

pub fn load_config() -> Result<()> {
    let path = "myconfig";  
    let _text = std::fs::read_to_string(path)
        .context(format!("Failed to read config file: {}", path))?;
        
    let key = "foo";
    bail!("Unknown key '{}'", key);
}
