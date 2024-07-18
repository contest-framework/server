Feature: run Contest without configuration

  Scenario: run
    When I run "contest"
    Then it prints
      """
      Contest is online, Ctrl-C to exit
      """
