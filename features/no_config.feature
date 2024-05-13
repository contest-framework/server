Feature: run Tertestrial without configuration

  Scenario: run
    When I run "tertestrial"
    Then it exits with this output
      """
      Error: Configuration file not found

      Tertestrial requires a configuration file named ".testconfig.json" in the current directory. Please run "tertestrial setup " to create one.
      """
