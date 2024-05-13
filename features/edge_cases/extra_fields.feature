Feature: client sends unknown fields in the command

  Rule: ignores additional fields in commands

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
      When I start Tertestrial
      Then it prints
        """
        Tertestrial is online, Ctrl-C to exit
        """

    Scenario: send a valid command with additional wrong fields
      When a client sends the command '{ "command": "testAll", "foo": 1, "bar": 2 }'
      Then it prints
        """
        executing: echo running all tests
        running all tests
        """
