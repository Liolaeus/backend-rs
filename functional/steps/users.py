from behave import given


@given('I create "{}" users')
def create_users(context, count):
    for i in range(int(count)):
        context.execute_steps(
            f"""
            I send a "POST" request to "/users" with body
            \{
                "name": "user{i}",
                "email": "user{i}@mail.com",
                "password": "password{i}"
            \}
            """
        )
