use clap::crate_version;
use clap::value_t_or_exit;
use clap::App;
use clap::{crate_name, SubCommand};
use std::path::PathBuf;

fn main() {
    let matches = App::new(crate_name!())
        .about("rename file for backup")
        // use crate_version! to pull the version number
        .version(crate_version!())
        .arg_from_usage("[backup target] 'a require path for backup'")
        .subcommand(
            SubCommand::with_name("restore")
                .arg_from_usage("[restore target] 'a require path for restore'"),
        )
        .get_matches();

    let (c, n) = if let Some(r) = matches.subcommand_matches("restore") {
        let full_path =
            std::fs::canonicalize(value_t_or_exit!(r.value_of("restore target"), PathBuf))
                .expect("Can't get full path.");
        let next_path = remove_extension(&full_path);
        (full_path, next_path)
    } else {
        let full_path =
            std::fs::canonicalize(value_t_or_exit!(matches.value_of("backup target"), PathBuf))
                .expect("Can't get full path.");
        let next_path = add_extension(&full_path);
        (full_path, next_path)
    };

    std::fs::rename(c, n).unwrap();
}

fn add_extension(current_path: &PathBuf) -> PathBuf {
    let s = current_path.to_str().unwrap().to_owned() + ".bk";
    PathBuf::from(s)
}

fn remove_extension(current_path: &PathBuf) -> PathBuf {
    let s = current_path.to_str().unwrap().to_owned();
    PathBuf::from(s.trim_end_matches(".bk"))
}
