Feature: using unavailable line

  Scenario: in a "testAll" command
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testAll",
            "run": "echo running all tests",
            "vars": [
              {
                "name": "fn_name",
                "source": "currentOrAboveLineContent",
                "filter": "\\bfn (\\w+)\\("
              }
            ]
          }
        ]
      }
      """
    And Tertestrial is running
    When receiving the command '{ "command": "testFile", "file": "test.rs" }'
    Then it prints
      """
      Error: cannot determine command for trigger: testFile test.rs
      Please make sure that this action is listed in your configuration file
      """

  Scenario: in a "testFile" command
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testFile",
            "files": "*.rs",
            "run": "echo running all tests",
            "vars": [
              {
                "name": "fn_name",
                "source": "currentOrAboveLineContent",
                "filter": "\\bfn (\\w+)\\("
              }
            ]
          }
        ]
      }
      """
    And Tertestrial is running
    When receiving the command '{ "command": "testFile", "file": "test.rs" }'
    Then it prints
      """
      Error: Line not available
      To use the current line in a variable, you need to use the "testFunction" action type that provides this data.
      """
