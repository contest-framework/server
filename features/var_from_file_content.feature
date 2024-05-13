Feature: define custom variables

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "tests the function that the cursor is in right now",
            "trigger": {
              "command": "testFunction",
              "file": "**/*.rs"
            },
            "vars": [
              {
                "name": "fn_name",
                "source": "currentOrAboveLineContent",
                "filter": "\\bfn (\\w+)\\("
              }
            ],
            "run": "echo cargo test {{fn_name}}"
          }
        ]
      }
      """
    And file "foo.rs" with content
      """
      // This is the source code file that the user has currently opened.

      pub fn my_func() {
        println!("This is a very simple function.");
      }
      """
    And I start Tertestrial
    Then it prints
      """
      Tertestrial is online, Ctrl-C to exit
      """

  Scenario: sending a matching file and location
    When a client sends the command '{ "command": "testFunction", "file": "foo.rs", "line": "5" }'
    Then it prints
      """
      executing: echo cargo test my_func
      cargo test my_func
      """
