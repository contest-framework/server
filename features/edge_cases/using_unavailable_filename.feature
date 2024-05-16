Feature: using unavailable data

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testAll",
            "run": "echo running all tests",
            "vars": [
              {
                "name": "file_without_ext",
                "source": "file",
                "filter": "^(.+)\\.ts$"
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
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      Error: Filename is not known
      To use the filename in a variable, you need to choose either the "testFile" or "testFunction" action type that provides this data.
      """
