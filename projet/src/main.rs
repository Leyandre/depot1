use projet::cli::CliArguments;
use projet::scan::scan;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let args = CliArguments::new();

    println!("{:?}", args.path());

    let music_files = scan(args.path());
    for music_file in music_files {

        let serialized = serde_json::to_string(&music_file).unwrap();
        print_type_of(&serialized);
        println!("{:?}", music_file);
    }
}