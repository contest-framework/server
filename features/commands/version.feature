Feature: display the installed version

  Scenario: short version
    When I run "tertestrial -V"
    Then it exits with this output
      """
      tertestrial 0.1.0
      """

  Scenario: long version
    When I run "tertestrial --version"
    Then it exits with this output
      """
      tertestrial 0.1.0
      """
