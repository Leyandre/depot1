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

pub fn find_title(path: &Path, t: Tag) -> &str {
    let bpath = path.to_string().clone();
    let mut clic : bool = true;
    let mut result = String::new();

    while clic {
        let n = bpath.pop();

        if n == "." {clic = false}
    }

    clic = true;

    while clic {
        let n = bpath.pop();

        if n != "/" {
            let result = format!("{}{}", bpath.pop(), result.to_str());
        } 
        else {clic = false}
    }
    
    result;
}


impl MusicFile {
    pub fn new(path: &Path) -> MusicFile {
        let n = "None";
        
        match Tag::read_from_path(&path) {
            Err(_why) => {
                MusicFile {
                    path: path.to_path_buf(),
                    artiste: n.to_string(),
                    titre: find_title(),
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
                        None => find_title(),
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