Feature: Backend configuration loading
  Test startup config errors.

  Scenario Outline: Startup fails when required GPG config variable is missing
    Given the environment variable "<var>" is unset
    When I start the backend
    Then startup fails
    And stderr contains "Configuration loading failed"

    Examples:
      | var            |
      | GPG_KEY_ID     |
      | GPG_PASSPHRASE |
      | GPG_HOME       |

  Scenario Outline: Startup fails when HTTP_PORT is invalid
    Given the environment variable "HTTP_PORT" is set to "<invalid_port>"
    When I start the backend
    Then startup fails
    And stderr contains "Configuration loading failed"

    Examples:
      | invalid_port |
      | abc          |
      | -1           |
      | 99999        |

  Scenario Outline: Startup succeeds with enough env vars
    Given the environment variable "<var>" is unset
    When I start the backend
    Then startup succeeds

    Examples:
      | var       |
      | HTTP_PORT |
      | HTTP_HOST |
