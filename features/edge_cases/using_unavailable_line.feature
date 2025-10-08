Feature: using unavailable line

  Scenario: in a "test-all" command
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-all",
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
    And Contest is running
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      Error: Filename is not known
      To use the filename in a variable, you need to choose either the "test-file" or "test-file-line" action type that provides this data.
      """

  Scenario: in a "test-file" command
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-file",
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
    And Contest is running
    When receiving the command '{ "command": "test-file", "file": "test.rs" }'
    Then it fails with this output
      """
      Error: Line not available

      To use the current line in a variable, you need to use the "test-file-line" action type that provides this data.
      """
