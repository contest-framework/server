Feature: display help

  Scenario Outline:
    When I run "contest <OPTION>"
    Then it exits with this output
      """
      server component for the continuous testing framework
      
      Usage: contest [COMMAND]
      
      Commands:
        debug  Print the received triggers from the pipe
        run    Run the given client-side trigger and exit
        init   Create an example configuration file
        start  Execute the received triggers from the pipe
        help   Print this message or the help of the given subcommand(s)
      
      Options:
        -h, --help     Print help
        -V, --version  Print version
      """

    Examples:
      | OPTION |
      | -h     |
      | --help |
