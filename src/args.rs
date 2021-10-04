//! command-line arguments

use super::errors::UserError;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

#[derive(Debug, PartialEq)]
pub enum Command {
    Normal,      // normal operation
    Debug,       // print the received commands from the pipe
    Run(String), // run the given command manually
    Setup,       // create a config file
}

pub fn parse<I>(argv: I) -> Result<Command, UserError>
where
    I: IntoIterator<Item = String>,
{
    let matches = define_args().get_matches_from(argv);
    match matches.subcommand() {
        ("debug", _) => Ok(Command::Debug),
        ("run", Some(run_options)) => Ok(Command::Run(
            run_options.value_of("url").unwrap().to_string(),
        )),
        ("setup", _) => Ok(Command::Setup),
        ("", _) => Ok(Command::Normal),
        (unknown, _) => panic!("unimplemented handler for CLI command '{}'", unknown),
    }
}

fn define_args() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("debug")
                .alias("d")
                .about("print the received commands from the pipe without running them"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .alias("r")
                .about("runs the given command manually")
                .arg(
                    Arg::with_name("command")
                        .help("pipe command to run")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("setup").about("create a config file"))
}

#[cfg(test)]
mod tests {

    mod parse {
        // use crate::args::{parse, UserError};

        // #[test]
        // fn no_args() {
        // let give = vec!["origins".into()];
        // match parse(give) {
        //     Err(UserError::NoCommandProvided { usage: _ }) => {}
        //     _ => panic!("unexpected"),
        // }
        // }

        mod run {
            use crate::args::{parse, Command};

            #[test]
            fn valid() {
                let give = vec![
                    "tertestrial".into(),
                    "run".into(),
                    r#"{"command": "testFile", "file": "src/probes/link_broken.rs" }"#.into(),
                ];
                assert_eq!(
                    parse(give),
                    Ok(Command::Run(
                        "https://github.com/origins-platform/origins-cli".into()
                    )),
                );
            }
        }

        mod debug {
            use crate::args::{parse, Command};

            #[test]
            fn long() {
                let give = vec!["origins".into(), "debug".into()];
                assert_eq!(parse(give), Ok(Command::Debug));
            }

            #[test]
            fn short() {
                let give = vec!["origins".into(), "d".into()];
                assert_eq!(parse(give), Ok(Command::Debug));
            }
        }
    }
}
