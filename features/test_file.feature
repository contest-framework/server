Feature: run all tests in a file

  Scenario: valid config file
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
    And I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """
    When a client sends the command '{ "command": "testFile", "file": "foo.rs" }'
    Then it prints
      """
      executing: echo testing file foo.rs
      testing file foo.rs
      """
