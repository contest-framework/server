Feature: display help

  @this
  Scenario: short version
    When I run "tertestrial -h"
    Then it exits with this output
      """
      tertestrial 0.0.2
      auto-run tests from within your code editor

      USAGE:
          tertestrial [SUBCOMMAND]
      """

  Scenario: long version
    When I run "tertestrial --help"
    Then it exits with this output
      """
      tertestrial 0.0.2
      """
