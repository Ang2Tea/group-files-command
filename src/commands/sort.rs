use clap::{Arg, ArgAction, Command};


pub struct SortArgs {
    pub input: Option<String>,
    pub output: Option<String>,
    pub force: bool,
    pub show_hidden: bool,
}

impl SortArgs {

    pub fn new() -> SortArgs {
        let matches = Command::new("Command for sorting files by extensions.")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Command for sorting files into folders")
            .arg(
                Arg::new("input_path")
                    .short('i')
                    .long("input")
                    .value_name("INPUT_PATH")
                    .action(ArgAction::Set)
                    .help("Set input path"),
            )
            .arg(
                Arg::new("output_path")
                    .short('o')
                    .long("output")
                    .value_name("OUTPUT_PATH")
                    .action(ArgAction::Set)
                    .help("Set output path"),
            )
            .arg(
                Arg::new("show_hidden")
                    .short('a')
                    .long("all")
                    .action(ArgAction::SetTrue)
                    .help("Show hidden files"),
            )
            .arg(
                Arg::new("force")
                    .short('f')
                    .long("force")
                    .action(ArgAction::SetTrue)
                    .help("Enable force modes"),
            )
            .get_matches();
    
    SortArgs{
            input: matches.get_one::<String>("input_path").map(|s| s.to_owned()),
            output: matches.get_one::<String>("output_path").map(|s| s.to_owned()),
            force : matches.get_flag("force"),
            show_hidden: matches.get_flag("show_hidden"),
        }
    }
}