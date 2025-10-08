Feature: repeat the last command

  Background:
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-all",
            "run": "echo running all tests"
          }
        ]
      }
      """
    And Contest is running

  Scenario: repeating the last command
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """
    When receiving the command '{ "command": "repeatTest" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """
    When receiving the command '{ "command": "repeatTest" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """
