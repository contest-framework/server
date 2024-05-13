Feature: client sends unknown fields in the command

  # TODO: wouldn't it be better to print at least a warning here?
  # Given that Tertestrial is doing a form of pattern matching on the received fields,
  # having extra fields should be an error.
  # Missing fields are already treated as an error.

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
        executing: echo running all tests
        running all tests
      """
