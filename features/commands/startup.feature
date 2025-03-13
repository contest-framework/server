Feature: Starting Contest

  Scenario:
    Given file ".contest.json" with content
      """
      {
        "actions": [
          {
            "type": "test-all",
            "run": "echo running all tests"
          }
        ]
      }
      """
    When I run "contest"
    Then it prints
      """
      Contest is online, Ctrl-C to exit
      """
