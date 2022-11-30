use projet::cli::CliArguments;
use projet::scan::scan;
use projet::search::search;
use std::path::Path;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn launch_scan(path : &Path) {
    let music_files = scan(path);
    for music_file in music_files {
        println!("{:?}", music_file);
    }
}

fn launch_search(kw : &String) {
    let mut vec : Vec<String> = Vec::new();
    let mut word : String = String::new();

    for i in kw.as_str().chars() {
        if i != ',' {
            word.push(i);
        }
        else {
            vec.push(word);
            word = String::new();
        }
    }
    vec.push(word);

    let mut music_f = search(vec);
    if music_f.len() == 0 {
        music_f.push("Aucun fichier ne correspond à la recherche".to_string())
    }
    for f in music_f {
        println!("{:?}",f);
    }

}

fn main() {
    let args = CliArguments::new();

    if args.scan() {
        println!("Scan succesfully started at : {:?}", args.path());
        
        launch_scan(args.path())

    } else if args.search() {
        println!("Search succesfully started for : {:?}", args.kwords());
        launch_search(args.kwords())
    }
}