use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Error;

// Crée un fichier #name.Json à l'endroit donné avec le texte donné
pub fn j_file(path: &Path, name: &str, text: String) -> Result<usize, Error> {
    // crée le chemin du fichier avec son nom et son extension
    let path_with_fname = format!("{}/{}.Json", path.to_str().unwrap(), name);

    // Crée le fichier
    let file = match File::create(path_with_fname) {
        Err(why) => panic!("{:?}", why),
        Ok(f) => f,
    };

    // Ecrit dans le fichier 
    let mut writer = BufWriter::new(&file);

    writer.write(text.as_bytes())
}