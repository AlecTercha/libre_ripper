use std::{
    fs::{
        File
    },
    io::{
        BufReader,
        Cursor,
        Read,
        Seek,
        SeekFrom
    },
    path::PathBuf
};
use hadris_udf::UdfVolume;

pub fn unpack(path: &PathBuf) -> std::io::Result<()> {
    let source = File::open(path).unwrap();
    let reader = BufReader::new(source);
    let data = UdfVolume::open(reader).unwrap();

    // Access root directory, and get the VIDEO_TS folder
    let root = data.root_dir().unwrap();
    let video_ts = root.find("VIDEO_TS").unwrap().icb;
    let video_ts = UdfVolume::read_directory(&data, &video_ts).unwrap();

    // Get the VMG (Video Manager) IFO contents
    let ifo_header = video_ts.find("VIDEO_TS.IFO").unwrap();
    let ifo_header = UdfVolume::read_file(&data, ifo_header).unwrap();
    let mut ifo_header = Cursor::new(ifo_header);

    // Seek to Menu VOB sector start pointer
    ifo_header.seek(SeekFrom::Start(0xC3))?;
    let mut buf = vec![0; 1];
    ifo_header.read_exact(&mut buf)?;

    println!("{:?}", buf);

    Ok(())
}