Feature: run all tests in a file

  Background:
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "type": "testFile",
            "files": "**/*.test.ts",
            "run": "echo testing file {{file}}"
          }
        ]
      }
      """
    And Contest is running

  Scenario: receiving a matching file
    When receiving the command '{ "command": "testFile", "file": "test/chars.test.ts" }'
    Then it prints
      """
      executing: echo testing file test/chars.test.ts
      testing file test/chars.test.ts
      """

  Scenario: receiving a file that doesn't match an existing rule
    When receiving the command '{ "command": "testFile", "file": "foo.go" }'
    Then it prints
      """
      Error: cannot determine command for trigger: testFile foo.go
      Please make sure that this action is listed in your configuration file
      """
    # ensure the server is still running and functional
    When receiving the command '{ "command": "testFile", "file": "test/chars.test.ts" }'
    Then it prints
      """
      executing: echo testing file test/chars.test.ts
      testing file test/chars.test.ts
      """
