Feature: using unavailable data

  Background:
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
    When I run "tertestrial"
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """

  Scenario: receiving a valid command
    When receiving the command '{ "command": "testFile", "file": "test.rs" }'
    Then it prints
      """
      Error: Line not available
      To use the current line in a variable, you need to use the "testFunction" action type that provides this data.
      """
