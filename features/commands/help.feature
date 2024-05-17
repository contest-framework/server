Feature: display help

  Scenario Outline:
    When I run "tertestrial <OPTION>"
    Then it exits with this output
      """
      auto-run tests from within your code editor

      Usage: tertestrial [COMMAND]

      Commands:
        debug  Start in debug mode
        run    Run the given client-side command and exit
        setup  Create an example configuration file
        start  Start in production mode
        help   Print this message or the help of the given subcommand(s)

      Options:
        -h, --help     Print help
        -V, --version  Print version
      """

    Examples:
      | OPTION |
      | -h     |
      | --help |
