Feature: run all tests in a file

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "trigger": {
              "command": "testFile",
              "file": "**/*.rs"
            },
            "run": "echo testing file {{file}}"
          }
        ]
      }
      """
    When I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """

  Scenario: sending a matching file
    When a client sends the command '{ "command": "testFile", "file": "foo.rs" }'
    Then it prints
      """
      executing: echo testing file foo.rs
      testing file foo.rs
      """

  Scenario: sending a file that doesn't match an existing rule
    When a client sends the command '{ "command": "testFile", "file": "foo.go" }'
    Then it prints
      """
      Error: cannot determine command for trigger: {"command": "testFile", "file": "foo.go" }
      """
