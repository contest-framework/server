Feature: define custom variables

  # This example describes situations where each source code file
  # (in this example TypeScript) has a corresponding unit test file.
  # The filename of the test files is the filename of the source code file
  # with "_test" appended.
  #
  # When the developer looks at a source file, and sends "test this file",
  # Tertestrial should test the corresponding test file
  # determined by the naming convention mentioned above.
  #
  # This is done with a custom variable that extracts the
  # filename of the source file without extension.

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
    When I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """

  Scenario: sending a matching file
    When a client sends the command '{ "command": "testFile", "file": "my_file.ts" }'
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
  Scenario: sending a mismatching file
    When a client sends the command '{ "command": "testFile", "file": "my_file.go" }'
    Then it prints
      """
      Error: cannot determine command for trigger: {"command": "testFile", "file": "my_file.go" }
      """

  #TODO: fix the missing space in the output
  Scenario: sending no file
    When a client sends the command '{ "command": "testFile" }'
    Then it prints
      """
      Error: cannot determine command for trigger: {"command": "testFile" }
      """
