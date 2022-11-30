use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use crate::musicfile::MusicFile;
use crate::cr_file::j_file;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

pub fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    if entry.file_name().to_str().unwrap()[0..1] == "."[0..1] {
        return true;
    }
    else {
        return false;
    }
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let save_path = Path::new("/Users/leyandre/Documents/Faculté/L3-Informatique/Semestre-1/Prog_Syst/depot1/projet/metadata");

    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap(); //TODO gérer l'erreur
        if is_supported(&entry) {
            music_files.push(MusicFile::new(entry.path()));

            let ser = serde_json::to_string(&music_files.last().unwrap()).unwrap();

            let mut fname = entry.file_name().to_str().unwrap().to_string().clone();

            for _i in entry.path().extension().unwrap().to_str().unwrap().chars() {
                fname.pop();
            }
            fname.pop();

            match j_file(save_path, fname.as_str(), ser) {
                Err(_e) => (),
                Ok(_o) => (),
            }
        }
    };
    music_files
}