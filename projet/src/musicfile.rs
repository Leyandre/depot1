use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use id3::{Tag, TagLike};

#[derive(Serialize, Deserialize, Debug)]
pub struct MusicFile {
    path: PathBuf,
    artiste: String,
    titre: String,
    album: String,
    annee: i32,
    duration: u32,
}


impl MusicFile {
    pub fn new(path: &Path) -> MusicFile {
        let n = "None";
        
        match Tag::read_from_path(&path) {
            Err(_why) => {
                MusicFile {
                    path: path.to_path_buf(),
                    artiste: n.to_string(),
                    titre: n.to_string(),
                    album: n.to_string(),
                    annee: 0,
                    duration: 0,
                }
            },
            Ok(tag_there) => {
                MusicFile {
                    path: path.to_path_buf(),
                    artiste: match tag_there.artist() {
                        None => n,
                        Some(it) => it,
                    }.to_string(),
                    titre: match tag_there.title(){
                        None => n,
                        Some(it) => it,
                    }.to_string(),
                    album: match tag_there.album(){
                        None => n,
                        Some(it) => it,
                    }.to_string(),
                    annee: match tag_there.year(){
                        None => 0,
                        Some(it) => it,
                    },
                    duration: match tag_there.duration(){
                        None => 0_u32,
                        Some(it) => it,
                    },
                }
            }
        }

    }
}