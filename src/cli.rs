use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .help("data file path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("list crypted env vars")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("command")
                .last(true)
                .allow_hyphen_values(true)
                .multiple(true),
        )
}
