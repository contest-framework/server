Feature: display the installed version

  Scenario: short version
    When I run "contest -V"
    Then it exits with this output
      """
      contest 0.3.0
      """

  Scenario: long version
    When I run "contest --version"
    Then it exits with this output
      """
      contest 0.3.0
      """
