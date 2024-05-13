Feature: client sends unknown fields in the command

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

  Scenario: send a valid command with additional wrong fields
    When receiving the command '{ "command": "testAll", "foo": 1, "bar": 2 }'
    Then it prints
      """
        Error: cannot parse command received from client: { "command": "testAll", "foo": 1, "bar": 2 }
        unknown field `foo`, expected one of `command`, `file`, `line` at line 1 column 29
      """
