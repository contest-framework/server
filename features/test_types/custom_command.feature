Feature: run a custom command sent by the client

  Scenario:
    Given Tertestrial is running
    When receiving the command '{ "command": "customCommand", "run": "echo custom command" }'
    Then it prints
      """
      executing: echo custom command
      custom command
      SUCCESS!
      """
    When receiving the command '{ "command": "repeatTest" }'
    Then it prints
      """
      executing: echo custom command
      custom command
      SUCCESS!
      """
