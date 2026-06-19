def add_context_user(c, user: dict):
    if not hasattr(c, "users"):
        c.users = {}
    c.users[user["email"]] = user


def get_context_user(c, email: str) -> dict:
    return getattr(c, "users", {}).get(email)


def must_get_context_user(c, email: str) -> dict:
    user = get_context_user(c, email)
    assert user is not None, f"user with email {email} not found in context"
    return user


# Returns true if 'subset' is a subset of 'set'
# Works for dicts & lists (and literals as part of the recursive implementation)
# For lists, checks if any of the set items exists as a subset of one of the target items
# Example:
#   is_subset({"a": 1}, {"a": 1, "b": 2}) == True
#   is_subset({"a": [1]}, {"a": [1, 2]) == True
#   is_subset({"a": [{"b": 1}]}, {"a": [{"b": 1, "c": 2}, {"d": 3}]}) == True
def is_subset(subset, set):
    if isinstance(subset, dict) and isinstance(set, dict):
        for key, value in subset.items():
            if key not in set or not is_subset(value, set[key]):
                return False
        return True
    elif isinstance(subset, list) and isinstance(set, list):
        for s_item in subset:
            if not any(is_subset(s_item, t_item) for t_item in set):
                return False
        return True
    else:
        return subset == set
