use std::{
    fs::{
        self, File
    }, io::BufReader, path::PathBuf
};
use hadris_udf::UdfVolume;

fn main() -> std::io::Result<()> {
    let path = &fs::canonicalize("../../../Downloads/786936799996 [imdb-tt0436339,disc-2].iso")?;

    println!("{}", check_source(path));

    Ok(())
}

fn check_source(path: &PathBuf) -> bool {
    match fs::exists(path) {
        Ok(result) =>
            if !result {
                println!("Path does not exist");
            }
        Err(error) => println!("Path can't be verified: {error}")
    }

    let metadata = fs::metadata(path).unwrap();

    if metadata.is_file() {
        let source = File::open(path).unwrap();
        let reader = BufReader::new(source);
        let data = UdfVolume::open(reader).unwrap();

        let root = data.root_dir().unwrap();
        for entry in root.entries() {
            println!("{} ({})", entry.name(), entry.is_dir());
        }
        return true;
    } else if metadata.is_dir() {
        return false;
    } else {
        return false;
    }
}