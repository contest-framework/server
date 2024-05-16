Feature: Starting Tertestrial

  Scenario:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "type": "testAll",
            "run": "echo running all tests"
          }
        ]
      }
      """
    When I run "tertestrial"
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """
