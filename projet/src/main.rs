use projet::cli::CliArguments;
use projet::scan::scan;
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

fn launch_search(_kw : &String) {
    ()
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