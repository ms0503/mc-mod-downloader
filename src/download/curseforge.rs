use crate::config::Mod;
use std::error::Error;
use std::path::Path;

pub async fn get_file(entry: Mod, dir: &Path) -> Result<(), Box<dyn Error>> {
    let Mod::CurseForge {
        file_id,
        name,
        project_id,
        ..
    } = entry
    else {
        unreachable!()
    };
    let out_filename = {
        let mut path = dir.to_path_buf();
        path.push(name);
        path
    };
    unimplemented!();
    Ok(())
}
