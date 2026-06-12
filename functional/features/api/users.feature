# Feature: Users API
#     Validate behavior of the Users endpoints

#   Background:
#     Given I restart the backend

#   Scenario Outline: Register rejects invalid user payloads
#     Given I set auth register payload user "<user>" and secret "<secret>"
#     When I send a "PUT" request to "/totp/register"
#     Then the response code is "400"
#     And the response body contains "user"

#     Examples:
#       | user | secret   |
#       | aa   | abcdef12 |
#       | !!!  | abcdef12 |


#   Scenario: Register same email twice
#     Given I set auth register payload user "<user>" and secret "<secret>"
#     When I send a "PUT" request to "/totp/register"
#     Then the response code is "400"
#     And the response body contains "user"

#     Examples:
#       | user | secret   |
#       | aa   | abcdef12 |
#       | !!!  | abcdef12 |
