Feature: define custom variables

  # In this example, unit tests for source files have names
  # following the pattern "{{file}}_test.ts".
  # When the client looks at a source file, and sends "test this file",
  # Tertestrial should test the corresponding test file.
  # This can be done with a custom variable that extracts the
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

  Scenario: sending a matching file and location
    When a client sends the command '{ "command": "testFile", "file": "foo.ts", "line": "23" }'
    Then it prints
      """
      executing: echo testing foo.test.ts
      testing foo.test.ts
      """
