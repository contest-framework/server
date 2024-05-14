Feature: run all tests

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "trigger": {
              "command": "testAll"
            },
            "run": "echo running all tests"
          }
        ]
      }
      """
    And I run "tertestrial debug"

  Scenario: receiving a valid command
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """
