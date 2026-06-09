Feature: /shop endpoints
  Test stock, basket and checkout flows.

  Background:
    Given I restart the backend
    And stock is empty

  Scenario Outline: Add stock succeeds
    When I send a "PUT" request to "/shop/stock" with body
      """
      [{"id": <id>, "amount": <amount>}]
      """
    Then the response code is "200"
    And the response json equals
      """
      {"result": "ok"}
      """

    Examples:
      | id    | amount |
      | 1     | 2      |
      | 2     | 7      |

  Scenario Outline: Listed stock contains seeded item
    Given stock is seeded with json body
      """
      [{"id": <id>, "amount": <amount>}]
      """
    When I send a "GET" request to "/shop/stock"
    Then the response code is "200"
    And the response stock has item "<id>" with amount "<amount>"

    Examples:
      | id | amount |
      | 1  | 2      |
      | 2  | 7      |

  Scenario Outline: Update basket returns expected error
    Given stock is seeded with json body
      """
      [{"id": <stock_id>, "amount": <stock_amount>}]
      """
    When I send a "POST" request to "/shop/basket" with body
      """
      {"id": <basket_id>, "basket": [{"id": <product_id>, "amount": <basket_amount>}]}
      """
    Then the response code is "400"
    And the response json equals
      """
      {"result": "<error_message>"}
      """

    Examples:
      | stock_id | stock_amount | basket_id | product_id | basket_amount | error_message                  |
      | 1        | 1            | 1         | 1          | 99            | oos                            |
      | 2        | 1            | 2         | 99         | 1             | product '99' does not exist    |

  Scenario: Update basket succeeds for seeded stock
    Given stock is seeded with json body
      """
      [{"id": 1, "amount": 10}, {"id": 2, "amount": 4}]
      """
    When I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 1, "amount": 3}, {"id": 2, "amount": 2}]}
      """
    Then the response code is "200"
    And the response json equals
      """
      {"result": "ok"}
      """


  Scenario: Baskets update oos
    Given stock is seeded with json body
      """
      [{"id": 1, "amount": 3}, {"id": 2, "amount": 100}]
      """
    When I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 1, "amount": 1}, {"id": 2, "amount": 50}]}
      """
    And the response code is "200"
    And I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 2, "amount": 160}]}
      """
    Then the response code is "400"
    And the response json has key "result" with value "oos"

  Scenario: Baskets update removes old content
    Given stock is seeded with json body
      """
      [{"id": 1, "amount": 100}, {"id": 2, "amount": 100}]
      """
    When I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 1, "amount": 10}, {"id": 2, "amount": 10}]}
      """
    And the response code is "200"
    And I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 2, "amount": 10}]}
      """
    And the response code is "200"
    When I check out basket "1"
    Then the response code is "200"
    And the response json equals
      """
      [{"id": 2, "amount": 10}]
      """

  Scenario: Checking out basket removes from stock
    Given stock is seeded with json body
      """
      [{"id": 1, "amount": 5}]
      """
    And I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 1, "amount": 3}]}
      """
    When I check out basket "1"
    Then the response code is "200"
    And the response json equals
      """
      [{"id": 1, "amount": 3}]
      """
    When I send a "GET" request to "/shop/stock"
    Then the response code is "200"
    And the response json equals
      """
      [{"id": 1, "amount": 2}]
      """

  Scenario: Check out empty basket
    Given stock is empty
    When I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": []}
      """
    Then the response code is "200"
    And the response json equals
      """
      {"result": "ok"}
      """
    When I check out basket "1"
    Then the response code is "200"
    And the response json equals
      """
      []
      """

  Scenario: Check out invalid basket
    Given stock is empty
    When I check out basket "1"
    Then the response code is "400"
    And the response json equals
      """
      {"message": "basket does not exist"}
      """

  Scenario: Multiple baskets
    Given stock is seeded with json body
      """
      [{"id": 1, "amount": 10}, {"id": 2, "amount": 10}]
      """
    When I send a "POST" request to "/shop/basket" with body
      """
      {"id": 1, "basket": [{"id": 1, "amount": 5}]}
      """
    And I send a "POST" request to "/shop/basket" with body
      """
      {"id": 2, "basket": [{"id": 1, "amount": 5}, {"id": 2, "amount": 5}]}
      """
    When I check out basket "2"
    Then the response code is "200"
    And the response json equals
      """
      [{"id": 1, "amount": 5}, {"id": 2, "amount": 5}]
      """



  Scenario: Checkout unknown basket
    Given stock is empty
    When I send a "POST" request to "/shop/checkout" with body
      """
      {"id": 99}
      """
    Then the response code is "400"
