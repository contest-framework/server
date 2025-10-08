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
    When receiving the command '{ "command": "quit" }'
    Then it prints
      """
      See you later!
      """
    And the server stops running
