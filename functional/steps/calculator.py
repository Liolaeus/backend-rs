from urllib.parse import quote_plus

from behave import when


@when('I call calculator with expression "{expr}"')
def step_call_calculator(context, expr):
    value = "" if expr == "<empty>" else expr
    encoded = quote_plus(value)
    path = f"/calculatrice?expr={encoded}"
    context.execute_steps(
        f'''\
        When I send a "GET" request to "{path}"
        '''
    )
