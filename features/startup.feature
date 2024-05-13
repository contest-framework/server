Feature: Starting Tertestrial

  Scenario:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "trigger": {
              "command": "testAll"
            },
            "run": "echo running all tests"
          }
        ]
      }
      """
    When I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """
