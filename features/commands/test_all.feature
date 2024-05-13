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
    And Tertestrial is running

  Scenario: sending a valid command
    When a client sends the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """
