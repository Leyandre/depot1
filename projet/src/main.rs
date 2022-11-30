use projet::cli::CliArguments;
use projet::scan::scan;
use projet::search::search;
use std::path::Path;

// Fonction se chargeant du scan d'un dossier
fn launch_scan(path : &Path) {
    let music_files = scan(path);
    for music_file in music_files {
        println!("{:?}", music_file);
    }
}

// Fonction se chargeant de la recherche de mots-clés dans le dossier metadata, reçoit un string de mot-sclés séparés par ","
fn launch_search(kw : &String) {
    // vecteur récupérant tous les mots-clés séparés 
    let mut vec : Vec<String> = Vec::new();

    // variable tampon pour récupérer tous les mots-clés
    let mut word : String = String::new();

    // Boucle qui lit le string lettres par lettres, et pose chaque nouvelle lettre dans word
    for i in kw.as_str().chars() {
        if i != ',' {
            word.push(i);
        }
        else {
            // si la lettre est "," c'est la fin du mot il est renvoyé à vec et word est nettoyé
            vec.push(word);
            word = String::new();
        }
    }
    // renvoie le dernier mot du string
    vec.push(word);

    // démarre la recherche de mots-clés
    let mut music_f = search(&vec);

    // Si aucun fichier ne contient tous les mots-clés
    if music_f.len() == 0 {
        // On affiche 
        music_f.push("Aucun fichier ne correspond à la recherche".to_string())
    }
    for f in music_f {
        // Sinon on affiche les fichiers
        println!("{:?}",f);
    }

}

fn main() {
    // Lecture des arguments 
    let args = CliArguments::new();

    // Si l'argument scan est présent lance le scan
    if args.scan() {
        println!("Scan succesfully started at : {:?}", args.path());
        
        launch_scan(args.path())

    } else if args.search() {
        // Si l'argument search est présent lance la recherche
        println!("Search succesfully started for : {:?}", args.kwords());
        launch_search(args.kwords())
    }
}