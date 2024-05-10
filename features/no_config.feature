Feature: run Tertestrial without configuration

  @this
  Scenario: run
    Given no configuration file
    When I run "tertestrial"
    Then it prints:
      """
      xxx
      """
