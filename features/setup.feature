Feature: create example config file as part of setup

  Scenario:
    When I run "tertestrial setup"
    Then it exits with no output
    And it creates file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "trigger": { "command": "testAll" },
            "run": "echo test all files"
          },

          {
            "trigger": {
              "command": "testFile",
              "file": "\\.rs$"
            },
            "run": "echo testing file {{file}}"
          },

          {
            "trigger": {
              "command": "testFunction",
              "file": "\\.ext$",
            },
            "run": "echo testing file {{file}} at line {{line}}"
          }
        ]
      }
      """
