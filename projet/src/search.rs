use std::path::Path;
use walkdir::{WalkDir};
use crate::musicfile::MusicFile;
use crate::scan::{is_hidden, is_supported};
use std::fs::File;
use std::io::Read;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["Json"];

pub fn search(vec_kwords: Vec<String>) -> Vec<String>{
    let nb_words : u8 = vec_kwords.len().try_into().unwrap();
    let path_to_meta_dir = Path::new("/Users/leyandre/Documents/Faculté/L3-Informatique/Semestre-1/Prog_Syst/depot1/projet/metadata");

    let mut musics_found : Vec<String> = Vec::new();

    let walker = WalkDir::new(path_to_meta_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();

        if is_supported(&entry) {
            let mut file = match File::open(entry.path()) {
                Err(why) => panic!("{:?}",why),
                Ok(f) => f,
            };
        
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("{:?}", file),
                Ok(_) => println!("lecture du fichier réussi")
            };

            let deser : MusicFile = serde_json::from_str(s.as_str()).unwrap();

            let mut nb_occur : u8 = 0;
            for i in &vec_kwords {
                if *i == deser.artiste().as_str() || *i == deser.titre().as_str() || *i == deser.album().as_str() {
                    nb_occur = nb_occur + 1;
                }
                else if *i == deser.annee().to_string() || *i == deser.duration().to_string() {
                    nb_occur = nb_occur + 1;
                }
            }

            if nb_occur == nb_words {
                musics_found.push(entry.file_name().to_str().unwrap().to_string().clone())
            }
        }
    }
    musics_found
}