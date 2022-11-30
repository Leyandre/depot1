use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use id3::{Tag, TagLike};

// Structure recevant les métadonnées d'un fichier mp3 lu
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
    // Création automatique d'un MusicFile
    pub fn new(path: &Path) -> MusicFile {
        // mot rapide en cas d'absence de valeur
        let n = "None";
        
        // Vérifie si le fichier mp3 contient bien un tag id3
        match Tag::read_from_path(&path) {
            // Si non, configure des valeurs par défaut
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
            // Si oui, récupère du tag les informations recherchées
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

    // Création manuelle d'un MusicFile
    pub fn set_new(p: &Path, a: String, t: String, al: String, an: i32, d: u32) -> MusicFile {
        MusicFile {
            path: p.to_path_buf(),
            artiste: a,
            titre: t,
            album: al,
            annee: an,
            duration: d,
        }
    }

    // récupère le nom de l'artiste
    pub fn artiste(&self) -> &String {
        &self.artiste
    }

    // récupère le nom du titre du morceau
    pub fn titre(&self) -> &String {
        &self.titre
    }

    // récupère le nom de l'album du morceau
    pub fn album(&self) -> &String {
        &self.album
    }

    // récupère l'année de sortie du morceau
    pub fn annee(&self) -> i32 {
        self.annee
    }

    // récupère la durée du morceau
    pub fn duration(&self) -> u32 {
        self.duration
    }
}