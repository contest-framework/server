Feature: run Tertestrial with configuration that defines commands

  @this
  Scenario: configuration defines commands
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "run all tests",
            "trigger": {
              "command": "testAll"
            },
            "run": "echo make test"
          }
        ],
        "options": {
          "beforeRun": {
            "clearScreen": false
          }
        }
      }
      """
    And I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """
    When a client sends the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo make test
      """
