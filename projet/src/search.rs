use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use crate::musicfile::MusicFile;
use crate::scan::{is_hidden};
use std::fs::File;
use std::io::Read;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["Json"];

// vérifie que c'est bien un fichier qui va être lu, de type .Json
fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

fn entry_reading_to_str(path: &Path, s: &mut String) {

    // ouvre le fichier 
    let mut file = match File::open(path) {
        Err(why) => panic!("{:?}",why),
        Ok(f) => f,
    };

    // prépare un string pour la lecture du fichier
    let mut string_here = String::new();

    // lecture du fichier
    match file.read_to_string(&mut string_here) {
        Err(why) => panic!("{:?}", why),
        Ok(_) => println!("lecture du fichier réussi")
    };

    // copie du fichier string dans un string hors de la fonction (évites les problèmes d'emprunt)
    *s = string_here
}

// Cherches le nombre de mots-clé présent dans les metadata du fichier
fn find_nb_wd_occur(v_kw: &Vec<String>, music_f: MusicFile) -> u8 {
    let mut nb_occur : u8 = 0;

    // Pour chaque mot-clé vérification de sa présence dans le fichier, rajoute 1 à l'occurence si oui
    for i in v_kw {
        if *i == music_f.artiste().as_str() || *i == music_f.titre().as_str() || *i == music_f.album().as_str() || *i == music_f.annee().to_string() || *i == music_f.duration().to_string(){
            nb_occur = nb_occur + 1;
        }
    }

    nb_occur
}

pub fn search(vec_kwords: &Vec<String>) -> Vec<String>{
    // retiens le nombre de mots-clés recherchés
    let nb_words : u8 = vec_kwords.len().try_into().unwrap();

    // Chemin vers le dossier contenant les metadata créées
    let path_to_meta_dir = Path::new("metadata");

    // vecteur retenant le nom de tous les fichiers contenant les mots-clés
    let mut musics_found : Vec<String> = Vec::new();

    let walker = WalkDir::new(path_to_meta_dir).into_iter();

    // Etude de tous les fichiers présent dans le dossier métadata
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();

        if is_supported(&entry) {
            
            // string contenant le fichier lu
            let mut st = String::new();
            entry_reading_to_str(entry.path(), &mut st);

            // transformation du string en struct MusicFile plus simple à lire
            let deser : MusicFile = serde_json::from_str(st.as_str()).unwrap();

            // Si le nombre d'occurence de mots-clés est égale au nombre de mots-clés recherchés alors le fichier est retenu dans musics_found
            if find_nb_wd_occur(vec_kwords, deser) == nb_words {
                musics_found.push(entry.file_name().to_str().unwrap().to_string().clone())
            }
        }
    }

    // renvoie tous les fichiers contenant les mots-clés
    musics_found
}

#[cfg(test)]
mod test_on_search {
    use std::path::Path;
    use crate::musicfile::MusicFile;
    use crate::search::find_nb_wd_occur;


    #[test]
    fn test_find_occur() {
        let mf = MusicFile::set_new(Path::new("metadata/filename"), "Miles Davis".to_string(), "titre".to_string(), "album".to_string(), 2020, 1343434_u32);

        let mut vk :Vec<String> = Vec::new();
        vk.push("Miles Davis".to_string());

        assert_eq!(find_nb_wd_occur(&vk, mf), 1_u8);
    }
}