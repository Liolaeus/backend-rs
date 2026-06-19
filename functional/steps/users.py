from behave import Given, given, step, then
import requests
from concurrent.futures import ThreadPoolExecutor

from steps.utils import add_context_user, must_get_context_user


@given('I create "{}" users')
def create_users(context, count):
    def create_user(i):
        resp = requests.post(
            f"{context.base_url}/users",
            headers=context.request_headers,
            json={
                "name": f"user{i}",
                "email": f"user{i}@mail.com",
                "password": f"password{i}",
            },
        )
        assert resp.status_code == 201, f"failed to create user: {resp.text}"
        return resp.json()

    with ThreadPoolExecutor() as executor:
        results = list(executor.map(create_user, range(int(count))))

    for user in results:
        add_context_user(context, user)


@Given('user "{email}" exists')
def step_user_exists(context, email):
    url = f"{context.base_url}/users"
    page = 1

    while True:
        resp = requests.get(url, params={"page": page, "page_size": 100}, timeout=5, verify=False)
        assert resp.status_code == 200, f"failed to fetch users: {resp.text}"

        users = resp.json().get("data", [])

        if not users:
            break
        for user in users:
            if user.get("email") == email:
                return
        page += 1

    raise AssertionError(f"user {email} not found")


@then('user "{email}" does not exist')
def step_user_not_exist(context, email):
    url = f"{context.base_url}/users"
    page = 1

    while True:
        resp = requests.get(url, params={"page": page, "page_size": 100}, timeout=5, verify=False)
        assert resp.status_code == 200, f"failed to fetch users: {resp.text}"

        users = resp.json().get("data", [])

        if not users:
            break
        for user in users:
            if user.get("email") == email:
                raise AssertionError(f"user {email} should not exist")
        page += 1


@step('I delete user "{email}"')
def step_delete_user(context, email):
    user = must_get_context_user(context, email)
    url = f"{context.base_url}/users/{user['uuid']}"

    resp = requests.delete(url, timeout=5, verify=False)
    assert resp.status_code == 204, f"failed to delete user: {resp.text}"
