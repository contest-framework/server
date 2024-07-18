Feature: client sends unknown fields in the command

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
    And Contest is running

  Scenario: send a valid command with additional wrong fields
    When receiving the command '{ "command": "testAll", "foo": 1, "bar": 2 }'
    Then it fails with this output
      """
Error: cannot parse command received from client: { "command": "testAll", "foo": 1, "bar": 2 }

unknown field `foo`, expected one of `command`, `file`, `line`, `run`
      """
