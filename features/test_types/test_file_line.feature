Feature: test only a specific function

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testFileLine",
            "files": "**/*.ts",
            "run": "echo testing file {{file}}:{{line}}"
          }
        ]
      }
      """
    And Contest is running

  Scenario: receiving a matching file and location
    When receiving the command '{ "command": "testFileLine", "file": "foo.ts", "line": 23 }'
    Then it prints
      """
      executing: echo testing file foo.ts:23
      testing file foo.ts:23
      """

  Scenario: receiving a matching file and no location
    When receiving the command '{ "command": "testFileLine", "file": "foo.ts" }'
    Then it fails with this output
      """
Error: invalid trigger received: { "command": "testFileLine", "file": "foo.ts" }

missing "line" field
      """

  Scenario: receiving a mismatching file
    When receiving the command '{ "command": "testFileLine", "file": "foo.go", "line": 23 }'
    Then it prints
      """
      Error: cannot determine command for trigger: testFileLine foo.go:23
      Please make sure that this action is listed in your configuration file
      """
    # ensure the server is still running and functional
    When receiving the command '{ "command": "testFileLine", "file": "foo.ts", "line": 23 }'
    Then it prints
      """
      executing: echo testing file foo.ts:23
      testing file foo.ts:23
      """
