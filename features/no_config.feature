Feature: run Tertestrial without configuration

  @this
  Scenario: run
    When I start Tertestrial
    Then it prints:
      """
      xxx
      """
