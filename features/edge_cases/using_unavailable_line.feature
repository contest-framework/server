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
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      Error: Filename is not known
      To use the filename in a variable, you need to choose either the "testFile" or "testFileLine" action type that provides this data.
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
    Then it fails with this output
      """
Error: Line not available

To use the current line in a variable, you need to use the "testFileLine" action type that provides this data.
      """
