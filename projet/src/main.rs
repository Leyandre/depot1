use projet::cli::CliArguments;
use projet::scan::scan;

fn main() {
    let args = CliArguments::new();

    println!("{:?}", args.path());

    let music_files = scan(args.path());
    for music_file in music_files {
        println!("{:?}", music_file);
    }
}