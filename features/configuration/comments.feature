Feature: comments in the configuration file

  Scenario: JSON file extension
    Given file ".testconfig.json" with content
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
    And Tertestrial is running
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """

  @this
  Scenario: JSON5 file extension
    Given file ".testconfig.json5" with content
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
    And Tertestrial is running
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      SUCCESS
      """
