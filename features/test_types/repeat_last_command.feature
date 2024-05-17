Feature: repeat the last command

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testAll",
            "run": "echo running all tests"
          }
        ]
      }
      """
    And Tertestrial is running

  Scenario: repeating the last command
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS!
      """
    When receiving the command '{ "command": "repeatTest" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS!
      """
    When receiving the command '{ "command": "repeatTest" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS!
      """
