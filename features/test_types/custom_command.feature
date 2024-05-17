Feature: run a custom command sent by the client

  Scenario:
    Given Tertestrial is running
    When receiving the command '{ "command": "custom", "run": "echo custom command" }'
    Then it prints
      """
      executing: echo custom command
      custom command
      """
