Feature: run all tests in a file

  Background:
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-file",
            "files": "**/*.test.ts",
            "run": "echo testing file {{file}}"
          }
        ]
      }
      """
    And Contest is running

  Scenario: receiving a matching file
    When receiving the command '{ "command": "test-file", "file": "test/chars.test.ts" }'
    Then it prints
      """
      executing: echo testing file test/chars.test.ts
      testing file test/chars.test.ts
      """

  Scenario: receiving a file that doesn't match an existing rule
    When receiving the command '{ "command": "test-file", "file": "foo.go" }'
    Then it prints
      """
      TRIGGER                | RUN
      test-file **/*.test.ts | echo testing file {{file}}
      Error: cannot determine command for trigger: test-file foo.go
      Please make sure that this action is listed in contest.json
      The current configuration is:
      Options:
      - beforeRun.clearScreen: false
      """
    # ensure the server is still running and functional
    When receiving the command '{ "command": "test-file", "file": "test/chars.test.ts" }'
    Then it prints
      """
      executing: echo testing file test/chars.test.ts
      testing file test/chars.test.ts
      """
