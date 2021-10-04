//! command-line arguments

use super::errors::TertError;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

#[derive(Debug, PartialEq)]
pub enum Command {
    Normal,      // normal operation
    Debug,       // print the received commands from the pipe
    Help,        // print the help screen
    Run(String), // run the given command manually
    Setup,       // create a config file
    Version,     // show the version
}

pub fn parse<I>(argv: I) -> Result<Command, TertError>
where
    I: IntoIterator<Item = String>,
{
    let matches = define_args().get_matches_from(argv);
    match matches.subcommand() {
        ("build-all", _) => Ok(Command::BuildAll),
        ("clone", Some(clone_options)) => Ok(Command::Clone {
            url: clone_options.value_of("url").unwrap().to_string(),
        }),
        ("lint-all", _) => Ok(Command::LintAll),
        ("setup-all", _) => Ok(Command::SetupAll),
        ("start", _) => Ok(Command::Start),
        ("server", _) => Ok(Command::Server),
        ("", _) => Err(UserError::NoCommandProvided {
            usage: matches.usage().into(),
        }),
        (unknown, _) => panic!("unimplemented handler for CLI command {}", unknown),
    }
}

fn define_args() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("build-all")
                .alias("ba")
                .about("builds all code bases in this mesh"),
        )
        .subcommand(
            SubCommand::with_name("clone")
                .alias("c")
                .about("clones a mesh repository")
                .arg(
                    Arg::with_name("url")
                        .help("Repository url to clone")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("lint-all")
                .alias("la")
                .about("check the repository configuration file for mistakes"),
        )
        .subcommand(
            SubCommand::with_name("setup-all")
                .alias("sa")
                .about("prepare to execute commands in subshells"),
        )
        .subcommand(SubCommand::with_name("server").about("starts the config server"))
        .subcommand(
            SubCommand::with_name("start")
                .alias("st")
                .about("start all apps in subshells"),
        )
}

#[cfg(test)]
mod tests {

    mod parse {
        use crate::cli::{parse, UserError};

        #[test]
        fn no_args() {
            let give = vec!["origins".into()];
            match parse(give) {
                Err(UserError::NoCommandProvided { usage: _ }) => {}
                _ => panic!("unexpected"),
            }
        }

        mod clone {
            use crate::cli::{parse, Command};

            #[test]
            fn valid() {
                let give = vec![
                    "origins".into(),
                    "clone".into(),
                    "https://github.com/origins-platform/origins-cli".into(),
                ];
                assert_eq!(
                    parse(give),
                    Ok(Command::Clone {
                        url: "https://github.com/origins-platform/origins-cli".into()
                    })
                );
            }
        }

        mod lint {
            use crate::cli::{parse, Command};

            #[test]
            fn long() {
                let give = vec!["origins".into(), "lint-all".into()];
                assert_eq!(parse(give), Ok(Command::LintAll));
            }

            #[test]
            fn short() {
                let give = vec!["origins".into(), "la".into()];
                assert_eq!(parse(give), Ok(Command::LintAll));
            }
        }

        // This cannot be tested with the current implementation
        // because Clap prints the error message and exits on its own for wrong commands
        // instead of returning an error.
        // #[test]
        // fn unknown() {
        //     let give = vec!["origins".into(), "zonk".into()];
        //     match parse_command_line(give) {
        //         Err(UserError::UnknownCommand { command, usage: _ }) => {
        //             assert_eq!(command, "zonk".to_string());
        //         }
        //         _ => panic!(),
        //     }
        // }
    }
}

// pub fn parse<I>(mut argv: I) -> Result<Command, TertError>
// where
//     I: Iterator<Item = String>,
// {
//     argv.next(); // skip argv[0]
//     let mut mode = Command::Normal;
//     loop {
//         match argv.next() {
//             None => return Ok(mode),
//             Some(command) => match command.as_str() {
//                 "debug" => mode = Command::Debug,
//                 "help" => mode = Command::Help,
//                 "run" => match argv.next() {
//                     Some(cmd) => mode = Command::Run(cmd),
//                     None => return Err(TertError::ArgsMissingOptionForRunCommand {}),
//                 },
//                 "setup" => mode = Command::Setup,
//                 "version" => mode = Command::Version,
//                 _ => return Err(TertError::ArgsUnknownCommand { command }),
//             },
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_no_args() {
//         let give = vec!["tertestrial".to_string()];
//         let want = Ok(Command::Normal);
//         assert_eq!(parse(give.into_iter()), want);
//     }

//     #[test]
//     fn parse_debug() {
//         let give = vec!["tertestrial".to_string(), "debug".to_string()];
//         let want = Ok(Command::Debug);
//         assert_eq!(parse(give.into_iter()), want);
//     }

//     #[test]
//     fn parse_run_with_arg() {
//         let give = vec![
//             "tertestrial".to_string(),
//             "run".to_string(),
//             "my command".to_string(),
//         ];
//         let want = Ok(Command::Run("my command".to_string()));
//         assert_eq!(parse(give.into_iter()), want);
//     }

//     #[test]
//     fn parse_run_without_arg() {
//         let give = vec!["tertestrial".to_string(), "run".to_string()];
//         let want = Err(TertError::ArgsMissingOptionForRunCommand {});
//         assert_eq!(parse(give.into_iter()), want);
//     }

//     #[test]
//     fn parse_unknown() {
//         let give = vec!["tertestrial".to_string(), "zonk".to_string()];
//         let want = Err(TertError::ArgsUnknownCommand {
//             command: "zonk".to_string(),
//         });
//         assert_eq!(parse(give.into_iter()), want);
//     }
// }
