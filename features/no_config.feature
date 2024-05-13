Feature: run Tertestrial without configuration

  Scenario: run
    When I run "tertestrial"
    Then it prints
      """
      Error: Configuration file not found
      """
    And it exits
