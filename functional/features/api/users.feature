@api @users
Feature: Users API

    Background:
        Given I restart the backend

    Scenario Outline: Register rejects invalid user payloads
        When I send a "POST" request to "/users" with body
            """
            {
                "name": "<user>",
                "email": "<email>",
                "password": "<password>"
            }
            """
        Then the response code is "400"

        Examples:
            | user     | email         | password |
            | aa       | test@mail.com | abcdef12 |
            | username | testmail.com  | abcdef12 |
            | username | test@mail.com | flop     |


    Scenario: Register same email twice fails
        And I send a "POST" request to "/users" with body
            """
            {
                "name": "user",
                "email": "user@email.com",
                "password": "flop1234"
            }
            """
        And the response code is "201"
        And I send a "POST" request to "/users" with body
            """
            {
                "name": "user",
                "email": "user@email.com",
                "password": "flop1234"
            }
            """
        Then the response code is "400"
        And the response body contains "email taken"
