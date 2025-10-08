Feature: run all tests

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

  Scenario: receiving a valid command
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """
