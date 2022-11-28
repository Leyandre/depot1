use std::path::Path;

pub fn j_file(path: &Path, name: &str, text: String) -> bool {
    let path_with_fname = format!("{}/{}.Json", path.to_str().unwrap(), name);

    let mut _file = match File::create(path_with_fname) {
        Err(why) => panic!("{:?}", why),
        Ok(file) => file,
    };

    match file.write_fmt(format!(file.to_str())) {
        Err(why) => false,
        Ok(p) => true,
    }
}