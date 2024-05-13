Feature: define custom variables

  # In this example, unit tests can be filtered by the name of the function under test.
  # When the client

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
    When I start Tertestrial
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

  Scenario: sending a matching file and mismatching location
    When a client sends the command '{ "command": "testFunction", "file": "foo.rs", "line": "0" }'
    Then it prints
      """
      Error: Did not find pattern \bfn (\w+)\( in file foo.rs
      """

# TODO: fix the panic documented by this test
# Scenario: sending a matching file and no location
#   When a client sends the command '{ "command": "testFunction", "file": "foo.rs" }'
#   Then it prints
#     """
#     Error: Did not find pattern \bfn (\w+)\( in file foo.rs
#     """
