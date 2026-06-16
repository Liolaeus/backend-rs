import json

import requests
from behave import given, then, when, step


@given("backend is running")
def step_backend_is_running(context):
    if not context.backend.is_running():
        context.backend.start()


@given("I restart the backend")
def step_backend_restart(context):
    if context.backend.is_running():
        context.backend.stop()
    context.backend.start()


@given('I set request header "{name}" to "{value}"')
def step_set_request_header(context, name, value):
    context.request_headers[name] = value


@given("I set request json body to")
def step_set_request_json_body(context):
    context.request_json = json.loads(context.text)


@when('I send a "{method}" request to "{path}"')
def step_send_request(context, method, path):
    url = f"{context.base_url}{path}"
    context.response = requests.request(
        method=method,
        url=url,
        headers=context.request_headers,
        json=context.request_json,
        timeout=5,
    )


@step('I send a "{method}" request to "{path}" with body')
def step_send_request_with_body(context, method, path):
    context.request_json = json.loads(context.text)
    step_send_request(context, method, path)


@step('I send a "{method}" request to "{path}" with query parameters')
def step_send_request_with_query_params(context, method, path):
    params = {row[0]: row[1] for row in context.table}
    url = f"{context.base_url}{path}"
    context.response = requests.request(
        method=method,
        url=url,
        headers=context.request_headers,
        params=params,
        timeout=5,
    )


@step('the response code is "{status_code}"')
def step_assert_status_code(context, status_code):
    assert context.response is not None, "no response captured"
    assert context.response.status_code == int(status_code), (
        f"expected status {status_code}, got {context.response.status_code}, "
        f"body={context.response.text}"
    )


@then("the response json equals")
def step_assert_json_equals(context):
    expected = json.loads(context.text)
    actual = context.response.json()
    # ignore json return list orders
    if isinstance(expected, list):
        for v in expected:
            assert v in actual
        return
    assert actual == expected, f"expected {expected}, got {actual}"


@then('the response body contains "{data}"')
def step_assert_body_contains(context, data):
    body = context.response.text
    assert data in body, f"expected '{data}' in body '{body}'"


@then('the response json has key "{key}" with value "{value}"')
def step_assert_json_key_value(context, key, value):
    payload = context.response.json()
    assert key in payload, f"missing key '{key}' in {payload}"
    assert str(payload[key]) == value, f"expected {key}={value}, got {payload[key]}"
