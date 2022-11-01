use clap::{crate_name, crate_version, App, Arg};
use std::{env::current_dir, path::PathBuf};

pub fn options() -> (PathBuf, PathBuf) {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .usage(format!("{} <INPUT> -o? <PATH>", crate_name!()).as_str())
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .help("Source image file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("PATH")
                .help("Set output file path"),
        )
        .get_matches();

    let cur = current_dir().unwrap();

    let input = cur.join(app.value_of("INPUT").unwrap());

    let output = match app.value_of("output") {
        Some(s) => cur.join(s),
        None => {
            let name = input.file_name().unwrap().to_os_string();
            let mut path = cur.join(&name);
            path.set_extension("icns");
            path
        }
    };

    (input, output)
}
