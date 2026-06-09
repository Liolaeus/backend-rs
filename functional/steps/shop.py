import json

import requests
from behave import given, then, when


@given("stock is empty")
def step_stock_is_empty(context):
    url = f"{context.base_url}/shop/stock"
    response = requests.get(url, timeout=5)
    assert response.status_code == 200, f"failed to list stock: {response.text}"
    assert response.json() == [], f"expected empty stock, got {response.json()}"


@given('stock contains item "{item_id}" with amount "{amount}"')
def step_seed_single_stock_item(context, item_id, amount):
    url = f"{context.base_url}/shop/stock"
    response = requests.put(
        url,
        json=[{"id": int(item_id), "amount": int(amount)}],
        timeout=5,
    )
    assert response.status_code == 200, f"failed to seed stock: {response.text}"


@given("stock is seeded with json body")
def step_seed_stock_json(context):
    payload = json.loads(context.text)
    url = f"{context.base_url}/shop/stock"
    response = requests.put(url, json=payload, timeout=5)
    assert response.status_code == 200, f"failed to seed stock: {response.text}"


@given('basket "{basket_id}" has item "{item_id}" with amount "{amount}"')
def step_seed_basket_item(context, basket_id, item_id, amount):
    url = f"{context.base_url}/shop/basket"
    payload = {
        "id": int(basket_id),
        "basket": [{"id": int(item_id), "amount": int(amount)}],
    }
    response = requests.post(url, json=payload, timeout=5)
    assert response.status_code == 200, f"failed to seed basket: {response.text}"


@given('basket "{basket_id}" has been checked out')
def step_checkout_basket_precondition(context, basket_id):
    url = f"{context.base_url}/shop/checkout"
    response = requests.post(url, json={"id": int(basket_id)}, timeout=5)
    assert response.status_code == 200, f"failed to checkout basket: {response.text}"


@when('I check out basket "{basket_id}"')
def step_checkout_basket(context, basket_id):
    url = f"{context.base_url}/shop/checkout"
    context.response = requests.post(url, json={"id": int(basket_id)}, timeout=5)


@then('the response stock has item "{item_id}" with amount "{amount}"')
def step_assert_stock_item(context, item_id, amount):
    payload = context.response.json()
    matches = [row for row in payload if str(row.get("id")) == item_id]
    assert matches, f"missing stock item id={item_id} in {payload}"
    assert str(matches[0].get("amount")) == amount, (
        f"expected id={item_id} amount={amount}, got {matches[0]}"
    )


@then('the response stock has item "{item_id}" with amount at least "{amount}"')
def step_assert_stock_item_min_amount(context, item_id, amount):
    payload = context.response.json()
    matches = [row for row in payload if str(row.get("id")) == item_id]
    assert matches, f"missing stock item id={item_id} in {payload}"

    actual = int(matches[0].get("amount"))
    expected_min = int(amount)
    assert actual >= expected_min, (
        f"expected id={item_id} amount>={expected_min}, got {actual}"
    )


@then("the response stock matches")
def step_assert_stock_matches(context):
    expected = json.loads(context.text)
    payload = context.response.json()
    actual_map = {str(item["id"]): item["amount"] for item in payload}
    assert actual_map == expected, f"expected {expected}, got {actual_map}"
