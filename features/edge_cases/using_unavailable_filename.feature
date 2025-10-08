Feature: using unavailable filename

  Background:
    Given file "contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-all",
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
    And Contest is running


  Scenario: receiving a valid command
    When receiving the command '{ "command": "test-all" }'
    Then it prints
      """
      Error: Filename is not known
      To use the filename in a variable, you need to choose either the "test-file" or "test-file-line" action type that provides this data.
      """
