import os
import subprocess
from pathlib import Path

from behave import given, then, when


@given('the environment variable "{name}" is unset')
def step_unset_env_var(context, name):
    if not hasattr(context, "launch_env"):
        context.launch_env = os.environ.copy()
    context.launch_env.pop(name, None)


@given('the environment variable "{name}" is set to "{value}"')
def step_set_env_var(context, name, value):
    if not hasattr(context, "launch_env"):
        context.launch_env = os.environ.copy()
    context.launch_env[name] = value


@when("I start the backend")
def step_start_backend(context):
    backend_root = Path(__file__).resolve().parents[2] / "backend"
    subprocess.run(["pkill", "backend"])

    proc = subprocess.Popen(
        ["cargo", "run", "--quiet"],
        cwd=backend_root,
        env=context.launch_env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        start_new_session=True,
    )

    try:
        out, err = proc.communicate(timeout=0.1)
        context.exit_code = proc.returncode
        context.stdout = out
        context.stderr = err
    except subprocess.TimeoutExpired:
        # timeout = backend started and communicate time outed waiting for a return code
        proc.terminate()
        out, err = proc.communicate()
        context.stdout = out
        context.stderr = err
        context.exit_code = 0


@then("startup fails")
def step_startup_fails(context):
    assert context.exit_code != 0, context.stderr


@then("startup succeeds")
def step_startup_succeeds(context):
    assert context.exit_code == 0, context.stderr


@then('stderr contains "{snippet}"')
def step_stderr_contains(context, snippet):
    assert snippet in context.stderr, (
        f"expected '{snippet}' in stderr '{context.stderr}'"
    )
