use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Error;

pub fn j_file(path: &Path, name: &str, text: String) -> Result<usize, Error> {
    let path_with_fname = format!("{}/{}.Json", path.to_str().unwrap(), name);
    println!("{}",path_with_fname);

    let file = match File::create(path_with_fname) {
        Err(why) => panic!("{:?}", why),
        Ok(f) => f,
    };

    let mut writer = BufWriter::new(&file);

    writer.write(text.as_bytes())
}