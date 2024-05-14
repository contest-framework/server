Feature: repeat the last command

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

  @this
  Scenario: repeating the last command
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS!
      Warning: cannot determine terminal size
      """
    When receiving the command '{ "command": "repeatTest" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS!
      """
