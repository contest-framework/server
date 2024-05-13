Feature: display help

  Scenario Outline:
    When I run "tertestrial <OPTION>"
    Then it exits with this output
      """
      tertestrial 0.0.2
      auto-run tests from within your code editor

      USAGE:
          tertestrial [SUBCOMMAND]

      FLAGS:
          -h, --help       Prints help information
          -V, --version    Prints version information

      SUBCOMMANDS:
          debug    print the received commands from the pipe without running them
          help     Prints this message or the help of the given subcommand(s)
          run      runs the given command manually
          setup    create a config file
      """

    Examples:
      | OPTION |
      | -h     |
      | --help |
