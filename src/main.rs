use std::fs;
mod libunpack;

fn main() -> std::io::Result<()> {
    let path = &fs::canonicalize("../../../Downloads/786936799996 [imdb-tt0436339,disc-2].iso")?;

    match fs::exists(path) {
        Ok(result) =>
            if !result {
                println!("Path does not exist");
            }
        Err(error) => println!("Path can't be verified: {error}")
    }

    let metadata = fs::metadata(path).unwrap();

    if metadata.is_file() {
        libunpack::unpack(path)?;
    } else if metadata.is_dir() {

    } else {

    }

    Ok(())
}