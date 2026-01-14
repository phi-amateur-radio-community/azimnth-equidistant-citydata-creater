// Copyright (c) 2026 St Rangeset
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/arg.rs
// This module handles command-line argument parsing.


use clap::{Arg, ArgAction, Command};

fn spawn_common_help(cmd: Command) -> Command {
    cmd
        .after_help("\nAuthor: St Rangeset <rangeset24@outlook.com>\n\nLicense:\n  Copyright (c) 2026 St Rangeset\n  Licensed under the GPLv3 or later License.")
        .arg_required_else_help(true)
}

fn build_cli() -> Command {
    spawn_common_help(
        Command::new("aecc-fst")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("This is AEC's FST creater.")
            .disable_version_flag(true)
            // Version
            .arg(
                Arg::new("version")
                    .long("version")
                    .short('v')
                    .help("Show version information")
                    .action(ArgAction::SetTrue),
            )
            // Builder
            .subcommand(spawn_common_help(
                Command::new("build")
                    .about("Manage configuration files and options")
                    .arg(
                        Arg::new("source-type")
                            .long("source-type")
                            .short('s')
                            .help("Source data type (e.g. geonames)")
                            .value_name("TYPE")
                            .default_value("geonames"),
                    )
                    .arg(
                        Arg::new("input")
                            .long("input")
                            .short('i')
                            .help("Source data path.")
                            .required(true)
                            .value_name("PATH"),
                    )
                    .arg(
                        Arg::new("output")
                            .long("output")
                            .short('o')
                            .help("Output path.")
                            .value_name("PATH")
                            .default_value("aecc_out"),
                    ),
            )),
    )
}

pub fn handle_cli() {
    let matches = build_cli().get_matches();
    if matches.get_flag("version") {
        println!(
            " {}\nCopyright (c) 2026 St Rangeset\nLicensed under the GPLv3 or later License.",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    match matches.subcommand() {
        Some(("build", sub_m)) => {
            let source_type = sub_m.get_one::<String>("source-type").unwrap();
            let input = sub_m.get_one::<String>("input").unwrap();
            let output = sub_m.get_one::<String>("output").unwrap();

            //
        }
        _ => {
            println!("Unknown Command");
        }
    }
}
