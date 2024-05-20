Feature: define a custom variable with a regex match of the file content

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "tests the function that the cursor is in right now",
            "type": "testFileLine",
            "files": "**/*.rs",
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
    And Tertestrial is running

  Scenario: receiving a matching file and location
    When receiving the command '{ "command": "testFileLine", "file": "foo.rs", "line": 5 }'
    Then it prints
      """
      executing: echo cargo test my_func
      cargo test my_func
      SUCCESS
      """

  Scenario: receiving a matching file and mismatching location prints an error and keeps running
    When receiving the command '{ "command": "testFileLine", "file": "foo.rs", "line": 1 }'
    Then it prints
      """
      Error: did not find pattern \bfn (\w+)\( in file foo.rs at line 1
      This is defined in file .testconfig.json.
      """
    # ensure the server is still running and functional
    When receiving the command '{ "command": "testFileLine", "file": "foo.rs", "line": 3 }'
    Then it prints
      """
      executing: echo cargo test my_func
      cargo test my_func
      """

  Scenario: receiving a matching file and no location
    When receiving the command '{ "command": "testFileLine", "file": "foo.rs" }'
    Then it fails with this output
      """
Error: cannot parse command received from client: { "command": "testFileLine", "file": "foo.rs" }

trigger "testFileLine" is missing field "line"
      """
