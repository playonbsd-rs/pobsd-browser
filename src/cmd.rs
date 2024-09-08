use clap::{Arg, ArgAction, Command};

pub fn get_args() -> Command {
    Command::new("pobsd-browser")
        .about("playonbsd database browser")
        .version("0.1.0")
        .arg(
            Arg::new("file")
                .action(ArgAction::Set)
                .conflicts_with_all(["url", "official"])
                .required_unless_present_any(["url", "official"])
                .conflicts_with("url")
                .long("file")
                .short('f')
                .help("path to playonbsd database"),
        )
        .arg(
            Arg::new("url")
                .action(ArgAction::Set)
                .conflicts_with_all(["file", "official"])
                .required_unless_present_any(["file", "official"])
                .conflicts_with("file")
                .long("url")
                .short('u')
                .help("url to playonbsd database"),
        )
        .arg(
            Arg::new("official")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["file", "url"])
                .required_unless_present_any(["file", "url"])
                .long("official")
                .short('o')
                .help("use official playonbsd database (via GitHub)"),
        )
        .arg(
            Arg::new("steam_ids")
                .action(ArgAction::Set)
                .long("steam_ids")
                .short('s')
                .help("file containing the steam ids of owned game"),
        )
}
