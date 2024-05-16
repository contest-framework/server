Feature: define a custom variable with a part of the filename

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "run all tests for a TS source file",
            "type": "testFile",
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
    And Tertestrial is running

  Scenario: receiving a matching file
    When receiving the command '{ "command": "testFile", "file": "my_file.ts" }'
    Then it prints
      """
      executing: echo testing my_file.test.ts
      testing my_file.test.ts
      """

  Scenario: receiving a mismatching file
    When receiving the command '{ "command": "testFile", "file": "my_file.go" }'
    Then it prints
      """
      Error: cannot determine command for trigger: testFile my_file.go
      Please make sure that this action is listed in your configuration file
      """

  Scenario: receiving no file
    When receiving the command '{ "command": "testFile" }'
    Then it prints
      """
      Error: cannot parse command received from client: { "command": "testFile" }
      trigger "testFile" is missing field "file".
      """
