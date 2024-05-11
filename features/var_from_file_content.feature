Feature: define custom variables

  Background:
    Given file ".testconfig.json" with content
      """
      {
        "actions": [
          {
            "desc": "individual RS unit test",
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
    When a client sends the command '{ "command": "testFunction", "file": "foo.rs", "line": "2" }'
    Then it prints
      """
      executing: echo cargo test my_func
      cargo test my_func
      """
