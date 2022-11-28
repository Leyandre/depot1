use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use crate::musicfile::MusicFile;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

fn is_hidden(entry: &DirEntry) -> bool {
    if entry.file_name().to_str().unwrap()[0..1] == "."[0..1] {
        return true;
    }
    else {
        return false;
    }
}

pub fn scan(path: &Path) -> Vec<MusicFile> {

    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap(); //TODO gérer l'erreur
        if is_supported(&entry) {
            music_files.push(MusicFile::new(entry.path()));

            let ser = serde_json::to_string(music_files.last()).unwrap();

            j_file(path, music_files.last().unwrap().titre.to_str, ser)
        }
    };
    music_files
}