import os
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

import requests


def before_all(context):
    context.backend = init_backend()
    context.backend.build()
    context.base_url = context.backend.base_url


def after_all(context):
    subprocess.run(
        ["pkill", "backend"],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=1,
    )


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
        subprocess.run(["pkill", "backend"])

        if not self.backend_path.exists():
            raise RuntimeError("cargo build failed")

    def is_running(self) -> bool:
        try:
            response = requests.get(f"{self.base_url}/shop/stock", timeout=0.05)
            return response.status_code < 500
        except requests.RequestException:
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
        self.process.terminate()
        self.process.kill()
        self.process = None


def init_backend() -> BackendManager:
    backend_root = Path(__file__).resolve().parents[1] / "backend"
    backend_binary = backend_root / "target" / "debug" / "backend"

    return BackendManager(
        backend_root=backend_root,
        backend_path=backend_binary,
        base_url="http://127.0.0.1:8080",
    )
