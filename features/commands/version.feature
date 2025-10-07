Feature: display the installed version

  Scenario Outline:
    When I run "contest <ARG>"
    Then it exits with this output
      """
      contest 0.4.0
      """

    Examples:
      | ARG       |
      | -V        |
      | --version |
