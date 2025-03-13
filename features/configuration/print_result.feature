Feature: configuring textual output of the test result

  Scenario: default value
    Given file ".contest.json" with content
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
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """

  Scenario: disable the textual output
    Given file ".contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-all",
            "run": "echo running all tests"
          }
        ],
        "options": {
          "afterRun": {
            "printResult": false
          }
        }
      }
      """
    And Contest is running
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """

  Scenario: enable the textual output
    Given file ".contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-all",
            "run": "echo running all tests"
          }
        ],
        "options": {
          "afterRun": {
            "printResult": true
          }
        }
      }
      """
    And Contest is running
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """
