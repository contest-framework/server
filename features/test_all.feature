Feature: run all tests

  Scenario: valid config file
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
    And I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """
    When a client sends the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """
