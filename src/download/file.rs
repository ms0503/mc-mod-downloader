use crate::config::Mod;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

pub async fn get_file(entry: Mod, dir: &Path) -> Result<(), Box<dyn Error>> {
    let Mod::File {
        name,
        url,
        ..
    } = entry
    else {
        unreachable!()
    };
    let out_filename = {
        let mut path = dir.to_path_buf();
        path.push(&name);
        path
    };
    let data = reqwest::get(url).await?.bytes().await?;
    let mut out_file = File::create(out_filename).await?;
    io::copy(&mut data.as_ref(), &mut out_file).await?;
    println!("[INFO] {}: Downloaded", name);
    Ok(())
}
