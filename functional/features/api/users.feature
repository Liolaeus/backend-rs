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


    Scenario: List users with pagination
        Given I create "20" users
        And I send a "GET" request to "/users" with query parameters
            | page      | 1 |
            | page_size | 3 |
        Then the response code is "200"
        And the response json equals
            """
            {
                "page": 1,
                "page_size": 3,
                "total": 20,
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
        Given I create "20" users
        And I send a "GET" request to "/users" with query parameters
            | page      | 2 |
            | page_size | 5 |
        Then the response code is "200"
        And the response json equals
            """
            {
                "page": 2,
                "page_size": 5,
                "total": 20,
                "page_elts": 5,
                "data": [
                    {
                        "name": "user5",
                        "email": "user5@mail.com"
                    },
                    {
                        "name": "user6",
                        "email": "user6@mail.com"
                    },
                    {
                        "name": "user7",
                        "email": "user7@mail.com"
                    },
                    {
                        "name": "user8",
                        "email": "user8@mail.com"
                    },
                    {
                        "name": "user9",
                        "email": "user9@mail.com"
                    }
                ]
            }
            """


    Scenario: List users with invalid pagination rejects the request
        Given I create "3" users
        And I send a "GET" request to "/users" with query parameters
            | page      | 0 |
            | page_size | 5 |
        Then the response code is "400"


    Scenario: List users when none exist
        And I send a "GET" request to "/users" with query parameters
            | page      | 1 |
            | page_size | 5 |
        Then the response code is "200"
        And the response json equals
            """
            {
                "page": 1,
                "page_size": 5,
                "total": 0,
                "page_elts": 0,
                "data": []
            }
            """
