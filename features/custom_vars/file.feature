Feature: define a custom variable with a part of the filename

  Background:
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "desc": "run all tests for a TS source file",
            "type": "test-file",
            "files": "**/*.ts",
            "vars": [
              {
                "name": "file_without_ext",
                "source": "file",
                "filter": "^(.+)\\.ts$"
              }
            ],
            "run": "echo testing {{file_without_ext}}.test.ts"
          }
        ]
      }
      """
    And Contest is running

  Scenario: receiving a matching file
    When receiving the command '{ "command": "test-file", "file": "my_file.ts" }'
    Then it prints
      """
      executing: echo testing my_file.test.ts
      testing my_file.test.ts
      """

  Scenario: receiving a mismatching file prints an error and keeps running
    When receiving the command '{ "command": "test-file", "file": "my_file.go" }'
    Then it prints
      """
      TRIGGER           | RUN
      test-file **/*.ts | echo testing {{file_without_ext}}.test.ts
      Error: cannot determine command for trigger: test-file my_file.go
      Please make sure that this action is listed in contest.json
      The current configuration is:
      Options:
      - beforeRun.clearScreen: false
      """
    # ensure the server is still running and functional
    When receiving the command '{ "command": "test-file", "file": "my_file.ts" }'
    Then it prints
      """
      executing: echo testing my_file.test.ts
      testing my_file.test.ts
      """

  Scenario: receiving no file
    When receiving the command '{ "command": "test-file" }'
    Then it prints
      """
      Error: invalid trigger received: { "command": "test-file" }
      missing "file" field
      """
