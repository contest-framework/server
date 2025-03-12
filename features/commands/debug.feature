Feature: run in debug mode

  Background:
    Given file ".contest.json" with content
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
    When I run "contest debug"
    Then it prints
      """
      using this configuration:
      TRIGGER | RUN
      TestAll | echo running all tests
      Options:
      - beforeRun.clearScreen: false
      """
    And it prints
      """
      Contest is online in debug mode, Ctrl-C to exit
      """

  Scenario: receiving a valid command
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      received from client: { "command": "testAll" }
      """
    When receiving the command '{ "command": "testAll" }'
    Then it prints
      """
      received from client: { "command": "testAll" }
      """
