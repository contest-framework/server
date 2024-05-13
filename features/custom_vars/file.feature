Feature: define a custom variable with a part of the filename

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "run all tests for a TS source file",
            "trigger": {
              "command": "testFile",
              "file": "**/*.ts"
            },
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
    And Tertestrial is running

  Scenario: receiving a matching file
    When receiving the command '{ "command": "testFile", "file": "my_file.ts" }'
    Then it prints
      """
      executing: echo testing my_file.test.ts
      testing my_file.test.ts
      """

  # TODO: this error message prints internal implementation details that probably
  # aren't helpful to the end user. Print something better here, like:
  #
  # You asked me to test file my_file.go.
  # However, you didn't specify how to test such files.
  Scenario: receiving a mismatching file
    When receiving the command '{ "command": "testFile", "file": "my_file.go" }'
    Then it prints
      """
      Error: cannot determine command for trigger: { "command": "testFile", "file": "my_file.go" }
      """

  Scenario: receiving no file
    When receiving the command '{ "command": "testFile" }'
    Then it prints
      """
      Error: cannot determine command for trigger: { "command": "testFile" }
      """
