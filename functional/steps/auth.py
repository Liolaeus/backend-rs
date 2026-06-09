import hashlib
from datetime import datetime, timezone

from behave import given


@given('I set auth register payload user "{user}" and secret "{secret}"')
def step_set_register_payload(context, user, secret):
    context.execute_steps(
        f'''\
        Given I set request json body to
          """
          {{"user": "{user}", "secret": "{secret}"}}
          """
        '''
    )


@given('I set auth payload password "{password}"')
def step_set_auth_payload(context, password):
    context.execute_steps(
        f'''\
        Given I set request json body to
          """
          {{"password": "{password}"}}
          """
        '''
    )


@given('I set auth payload from secret "{secret}"')
def step_set_auth_payload_from_secret(context, secret):
    now_utc = datetime.now(timezone.utc).strftime("%Y%m%d-%H%M")
    password = hashlib.sha256(f"{secret}{now_utc}".encode("utf-8")).hexdigest()[:16]
    step_set_auth_payload(context, password)
