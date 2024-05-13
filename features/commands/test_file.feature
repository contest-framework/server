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
    And Tertestrial is running

  Scenario: receiving a matching file
    When receiving the command '{ "command": "testFile", "file": "foo.rs" }'
    Then it prints
      """
      executing: echo testing file foo.rs
      testing file foo.rs
      """

  Scenario: receiving a file that doesn't match an existing rule
    When receiving the command '{ "command": "testFile", "file": "foo.go" }'
    Then it prints
      """
      Error: cannot determine command for trigger: { "command": "testFile", "file": "foo.go" }
      """
