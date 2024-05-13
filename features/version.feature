Feature: display the installed version

  Scenario: short version
    When I run "tertestrial -V"
    Then it prints
      """
      tertestrial 0.0.2
      """

  Scenario: long version
    When I run "tertestrial --version"
    Then it prints
      """
      tertestrial 0.0.2
      """
