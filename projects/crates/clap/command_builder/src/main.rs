use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("My Program")
    .version("1.0.0")
    .author("Alice Adams")
    .about("This is my program")
    // Example arg that will set a string
    .arg(
        Arg::new("input")
        .help("Set the input to use")
        .short('i')
        .long("input")
        .action(ArgAction::Set)
    )
    // Example arg that will count its occurrences
    .arg(
        Arg::new("verbose")
        .help("Set the verbosity level")
        .short('v')
        .long("verbose")
        .action(ArgAction::Count)
    )
    .after_help("Longer explanation")
    .get_matches();

    // Process the "input" arg string 
    let x = matches.get_one::<String>("input");
    println!("input: {:?}", x);

    // Process the "verbose" arg count
    let x = matches.get_count("verbose");
    println!("verbose: {}", x);
        
}
