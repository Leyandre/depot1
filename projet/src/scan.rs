use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use crate::musicfile::MusicFile;
use crate::cr_file::j_file;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

// vérifie que c'est bien un fichier qui va être lu, de type .mp3
fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

// Vérifie si le fichier lu est un fichier caché si oui, ne le lis pas
pub fn is_hidden(entry: &DirEntry) -> bool {
    if entry.file_name().to_str().unwrap()[0..1] == "."[0..1] {
        return true;
    }
    else {
        return false;
    }
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    // chemin d'accès au dossier contenant les metadata
    let save_path = Path::new("metadata");

    // vecteur contenant tous les struct de metadata des fichiers
    let mut music_files: Vec<MusicFile> = Vec::new();

    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();

        if is_supported(&entry) {
            // Si le fichier est lisible, création du MusicFile avec les metadonnées du fichier
            music_files.push(MusicFile::new(entry.path()));

            // sérialisation du struct de metadonnées pour les écrire dans un fichier json
            let ser = serde_json::to_string(&music_files.last().unwrap()).unwrap();

            // Récupération du nom pour le fichier json
            let mut fname = entry.file_name().to_str().unwrap().to_string().clone();

            // Retrait de l'extension .mp3 pour le nom du fichier de metadata
            for _i in entry.path().extension().unwrap().to_str().unwrap().chars() {
                fname.pop();
            }
            fname.pop();

            // Création du fichier
            match j_file(save_path, fname.as_str(), ser) {
                Err(_e) => (),
                Ok(_o) => (),
            }
        }
    };

    // renvoie le vecteur contenant tous les fichiers scannés
    music_files
}