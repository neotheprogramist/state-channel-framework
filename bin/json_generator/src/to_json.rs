use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub async fn save_to_file<T: Serialize>(path: String, data: &T) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
