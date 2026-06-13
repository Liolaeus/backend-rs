Feature: Calculator API
    Test /calculatrice endpoint for valid and invalid requests.

    Background:
        Given backend is running

    Scenario Outline: Valid expressions
        When I call calculator with expression "<expr>"
        Then the response code is "200"
        And the response body contains "<expected>"

        Examples:
            | expr     | expected |
            | 1+2      | 3        |
            | (3+2)*3  | 15       |
            | 1/2      | 0.5      |
            | ------32 | 32       |
            | 2*3+10/2 | 11       |

    Scenario Outline: Reject invalid expressions
        When I call calculator with expression "<expr>"
        Then the response code is "400"
        And the response body contains "<err>"

        Examples:
            | expr        | err                          |
            | <empty>     | Unexpected token: Eof        |
            | (3-2        | Expected closing parenthesis |
            | (1+1)/(1-1) | division by zero             |
