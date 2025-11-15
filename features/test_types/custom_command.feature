Feature: run a custom command sent by the client

  Scenario:
    Given Contest is running
    When receiving the command '{ "command": "custom-command", "run": "echo custom command" }'
    Then it prints
      """
      executing: echo custom command
      custom command
      SUCCESS
      """
    When receiving the command '{ "command": "repeat-test" }'
    Then it prints
      """
      executing: echo custom command
      custom command
      SUCCESS
      """
