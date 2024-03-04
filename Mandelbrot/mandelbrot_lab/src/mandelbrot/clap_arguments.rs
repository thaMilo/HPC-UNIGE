use clap::{command, Arg};

pub fn get_clap_arguments() -> clap::ArgMatches {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("minx").short('x').long("minx").default_value("-2"))
        .arg(Arg::new("maxx").short('X').long("maxx").default_value("1"))
        .arg(Arg::new("miny").short('y').long("miny").default_value("-1"))
        .arg(Arg::new("maxy").short('Y').long("maxy").default_value("1"))
        .arg(
            Arg::new("resolution")
                .short('r')
                .long("resolution")
                .default_value("1000"),
        )
        .arg(
            Arg::new("degree")
                .short('d')
                .long("degree")
                .default_value("2"),
        )
        .arg(
            Arg::new("iterations")
                .short('i')
                .long("iterations")
                .default_value("1000"),
        )
        .arg(
            Arg::new("sequential-rust")
            .default_value("no_exec")
        )
        .arg(
            Arg::new("metal")
            .default_value("no_exec")
        )
        .arg(
            Arg::new("visualize")
                .default_value("no_path"),
        )
        .get_matches();
    matches
}
