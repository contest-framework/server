Feature: run Tertestrial with configuration that defines commands

  Scenario: configuration defines commands
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "unit test",
            "trigger": {
              "command": "testFunction",
              "file": "**/*.test.ts"
            },
            "run": "echo testing {{file}}:{{line}}"
          }
        ]
      }
      """
    And I start Tertestrial
    Then it prints:
      """
      Tertestrial is online, Ctrl-C to exit
      """
    When a client sends the command
      """
      {
        oeu
      }
      """
