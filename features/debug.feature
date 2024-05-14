Feature: run all tests

  Background:
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
    When I run "tertestrial debug"
    Then it prints
      """
      using this configuration:
      """
    And it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """

  @this
  Scenario: receiving a valid command
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      executing: echo running all tests
      running all tests
      """
