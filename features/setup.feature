Feature: create example config file as part of setup

  @this
  Scenario:
    When I run "tertestrial setup"
    Then it exits with no output
    And it creates file ".tertestrial.json" with content
      """
      """
