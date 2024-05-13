Feature: create example config file as part of setup

  @this
  Scenario:
    When I run "tertestrial setup"
    Then it exits with this output
      """
      tertestrial 0.0.2
      """
