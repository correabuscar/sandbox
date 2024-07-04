use clap::{Arg, Command, error::ErrorKind};

fn main() {
    let app = Command::new("MyApp")
        .arg(Arg::new("username")
            .short('u')
            .long("username")
            .value_name("USERNAME")
            .help("Sets the username")
            .num_args(1)
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("password")
            .short('p')
            .long("password")
            .value_name("PASSWORD")
            .help("Sets the password")
            .num_args(1)
            .value_parser(clap::value_parser!(String)));

    let matches_result = app.try_get_matches_from(std::env::args());

    match matches_result {
        Ok(matches) => {
            if let Some(username) = matches.get_one::<String>("username") {
                println!("Username: {}", username);
            }
            if let Some(password) = matches.get_one::<String>("password") {
                println!("Password: {}", password);
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::UnknownArgument {
                //FIXME: how to list all bad args, not only the first!
                eprintln!("Error: Unknown argument(s): {}", e);
            } else {
                eprintln!("Error: {}", e);
            }
            std::process::exit(1);
        }
    }
}

