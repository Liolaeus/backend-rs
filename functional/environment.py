import os
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

import requests


REQUIRED_BACKEND_ENV_VARS = (
    "GPG_KEY_ID",
    "GPG_PASSPHRASE",
    "GPG_HOME",
)


def before_all(context):
    assert_required_backend_env_vars_set()
    context.backend = init_backend()
    context.backend.build()
    context.base_url = context.backend.base_url


def before_scenario(context, scenario):
    context.request_headers = {}
    context.request_json = None
    context.response = None


@dataclass
class BackendManager:
    backend_root: Path
    backend_path: Path
    base_url: str
    process: Optional[subprocess.Popen] = None

    def build(self) -> None:
        subprocess.run(
            ["cargo", "build"],
            cwd=self.backend_root,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=30,
        )

        if not self.backend_path.exists():
            raise RuntimeError("cargo build failed")

    def is_running(self) -> bool:
        try:
            response = requests.get(f"{self.base_url}/shop/stock", timeout=0.05)
            return response.status_code < 500
        except requests.RequestException as e:
            return False

    def start(self) -> None:
        if self.process is not None and self.process.poll() is None:
            return
        if self.is_running():
            raise RuntimeError("backend is already running")

        env = os.environ.copy()

        self.process = subprocess.Popen(
            [str(self.backend_path)],
            cwd=self.backend_root,
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            self.process.communicate(timeout=0.05)
        except subprocess.TimeoutExpired:
            return
        else:
            raise Exception("Backend did not start")

    def stop(self) -> None:
        if self.process is None:
            return
        # if self.process.poll() is None:
        self.process.terminate()
        self.process.kill()
        self.process = None


def init_backend() -> BackendManager:
    backend_root = Path(__file__).resolve().parents[1]
    binary_name = "backend"
    backend_binary = backend_root / "target" / "debug" / binary_name

    return BackendManager(
        backend_root=backend_root,
        backend_path=backend_binary,
        base_url="http://127.0.0.1:8080",
    )


def assert_required_backend_env_vars_set() -> None:
    missing = [name for name in REQUIRED_BACKEND_ENV_VARS if not os.getenv(name)]
    if missing:
        raise RuntimeError(
            "missing required env vars for backend startup: "
            + ", ".join(missing)
            + ". Set them before running behave."
        )
