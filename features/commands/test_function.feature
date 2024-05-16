Feature: test only a specific function

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testFunction",
            "files": "**/*.ts",
            "run": "echo testing file {{file}}:{{line}}"
          }
        ]
      }
      """
    And Tertestrial is running

  Scenario: receiving a matching file and location
    When receiving the command '{ "command": "testFunction", "file": "foo.ts", "line": "23" }'
    Then it prints
      """
      executing: echo testing file foo.ts:23
      testing file foo.ts:23
      """

  # TODO: fix the wrong behavior documented by this test
  Scenario: receiving a matching file and no location
    When receiving the command '{ "command": "testFunction", "file": "foo.ts" }'
    Then it prints
      """
      executing: echo testing file foo.ts:{{line}}
      testing file foo.ts:{{line}}
      """

  Scenario: receiving a mismatching file
    When receiving the command '{ "command": "testFunction", "file": "foo.go", "line": "23" }'
    Then it prints
      """
      Error: cannot determine command for trigger: test foo.go:23
      Please make sure that this trigger is listed in your configuration file
      """
