@api @users
Feature: Users API

    Background:
        Given a clean install

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


    Scenario: List users with pagination
        Given I create "6" users
        And I send a "GET" request to "/users" with query parameters
            | page | page_size |
            | 1    | 3         |
        Then the response code is "200"
        And the response contains
            """
            {
                "page": 1,
                "page_size": 3,
                "total": 6,
                "page_elts": 3,
                "data": [
                    {
                        "name": "user0",
                        "email": "user0@mail.com"
                    },
                    {
                        "name": "user1",
                        "email": "user1@mail.com"
                    },
                    {
                        "name": "user2",
                        "email": "user2@mail.com"
                    }
                ]
            }
            """


    Scenario: List a later page of users
        Given I create "6" users
        And I send a "GET" request to "/users" with query parameters
            | page | page_size |
            | 2    | 3         |
        Then the response code is "200"
        And the response contains
            """
            {
                "page": 2,
                "page_size": 3,
                "total": 6,
                "page_elts": 3,
                "data": [
                    {
                        "name": "user3",
                        "email": "user3@mail.com"
                    },
                    {
                        "name": "user4",
                        "email": "user4@mail.com"
                    },
                    {
                        "name": "user5",
                        "email": "user5@mail.com"
                    }
                ]
            }
            """


    Scenario: List users with invalid pagination rejects the request
        Given I create "3" users
        And I send a "GET" request to "/users" with query parameters
            | page | page_size |
            | 0    | 5         |
        Then the response code is "400"


    Scenario: List users when none exist
        And I send a "GET" request to "/users" with query parameters
            | page | page_size |
            | 1    | 5         |
        Then the response code is "200"
        And the response contains
            """
            {
                "page": 1,
                "page_size": 5,
                "total": 0,
                "page_elts": 0,
                "data": []
            }
            """

    Scenario: Delete a user
        Given I create "1" users
        And user "user0@mail.com" exists

        When I delete user "user0@mail.com"

        Then user "user0@mail.com" does not exist
