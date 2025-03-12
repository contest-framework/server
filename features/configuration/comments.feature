Feature: comments in the configuration file

  Scenario: JSON file extension
    Given file "contest.json" with content
      """
      {
        "actions": [
          // run all tests
          {
            "type": "testAll",
            "run": "echo running all tests"
          }
        ]
      }
      """
    And Contest is running
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """

  Scenario: JSON5 file extension
    Given file "contest.json5" with content
      """
      {
        "actions": [
          // run all tests
          {
            "type": "testAll",
            "run": "echo running all tests"
          }
        ]
      }
      """
    And Contest is running
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """
