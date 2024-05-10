//! command-line arguments

use super::Result;
use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand};

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Normal,      // normal operation
    Debug,       // print the received commands from the pipe
    Run(String), // run the given command manually
    Setup,       // create a config file
}

pub fn parse<I>(argv: I) -> Result<Command>
where
    I: IntoIterator<Item = String>,
{
    match define_args().get_matches_from(argv).subcommand() {
        ("debug", _) => Ok(Command::Debug),
        ("run", Some(run_options)) => Ok(Command::Run(
            run_options.value_of("command").unwrap().to_owned(),
        )),
        ("setup", _) => Ok(Command::Setup),
        ("", _) => Ok(Command::Normal),
        (unknown, _) => panic!("unimplemented handler for CLI command '{}'", unknown),
    }
}

fn define_args() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
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
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("setup").about("create a config file"))
}

#[cfg(test)]
mod tests {

    mod parse {
        use crate::args::{parse, Command};

        #[test]
        fn no_args() {
            let give = vec!["tertestrial".into()];
            let have = parse(give);
            assert_eq!(have, Ok(Command::Normal));
        }

        mod run {
            use crate::args::{parse, Command};

            #[test]
            fn valid() {
                let give = vec![
                    "tertestrial".into(),
                    "run".into(),
                    r#"{"command": "testFile", "file": "src/probes/link_broken.rs" }"#.into(),
                ];
                let want = Ok(Command::Run(
                    r#"{"command": "testFile", "file": "src/probes/link_broken.rs" }"#.into(),
                ));
                assert_eq!(parse(give), want);
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
