Feature: Auth API
  Validate TOTP register and auth behaviors.

  Background:
    Given I restart the backend

  Scenario Outline: Register rejects invalid user payloads
    Given I set auth register payload user "<user>" and secret "<secret>"
    When I send a "PUT" request to "/totp/register"
    Then the response code is "400"
    And the response body contains "user"

    Examples:
      | user | secret   |
      | aa   | abcdef12 |
      | !!!  | abcdef12 |

  Scenario: Authenticate fails without X-User header
    Given I set auth payload password "abcd1234"
    When I send a "POST" request to "/totp/auth"
    Then the response code is "400"
    And the response json equals
      """
      {"message": "missing X-User header"}
      """

  Scenario Outline: Authentication succeeds
    Given I set auth register payload user "<user>" and secret "<secret>"
    When I send a "PUT" request to "/totp/register"
    Then the response code is "200"
    And the response json equals
      """
      {"result": "ok"}
      """
    Given I set request header "X-User" to "<user>"
    And I set auth payload from secret "<secret>"
    When I send a "POST" request to "/totp/auth"
    Then the response code is "200"
    And the response json equals
      """
      {"result": "ok"}
      """

    Examples:
      | user  | secret       |
      | user1 | abcdef12     |
      | user2 | passWORD99   |
      | user3 | AAAABBBB1234 |
      | user4 | z9y8x7w6     |

  Scenario: Authenticate fails with wrong password
    Given I set auth register payload user "user9" and secret "abcd1234"
    When I send a "PUT" request to "/totp/register"
    Then the response code is "200"
    And the response json equals
      """
      {"result": "ok"}
      """
    Given I set request header "X-User" to "user9"
    And I set auth payload password "aaaaaaaa"
    When I send a "POST" request to "/totp/auth"
    Then the response code is "401"
    And the response json equals
      """
      {"result": "unauthorized"}
      """
