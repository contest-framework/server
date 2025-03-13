Feature: create example config file as part of setup

  Scenario:
    When I run "contest setup"
    Then it exits with this output
      """
      Created config file ".contest.json"
      """
    And it creates file ".contest.json" with content
      """
      {
        "$schema": "https://raw.githubusercontent.com/contest-framework/server/refs/heads/main/documentation/schema.json",
        "actions": [
          {
            "type": "test-all",
            "run": "echo test all files"
          },
          {
            "type": "test-file",
            "file": "**/*.ext",
            "run": "echo testing file {{file}}"
          },
          {
            "type": "test-file-line",
            "file": "**/*.ext",
            "run": "echo testing file {{file}} at line {{line}}"
          }
        ],
        "options": {
          "beforeRun": {
            "clearScreen": false,
            "newlines": 2
          },
          "afterRun": {
            "newlines": 1,
            "indicatorLines": 2,
            "indicatorBackground": true,
            "printResult": true
          }
        }
      }
      """
