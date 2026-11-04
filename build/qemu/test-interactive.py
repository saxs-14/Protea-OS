#!/usr/bin/env python3
"""Boot the PC image in QEMU and exercise the desktop sign-in flow."""

import argparse
import json
import os
import queue
import re
import shutil
import socket
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path


CRASH_PATTERN = re.compile(
    r"reported baselines|g_object_ref: assertion|Unable to create Cairo image surface|"
    r"segfault|Segmentation fault|protea-desktop.*panicked|Failed to open display|"
    r"Kernel panic|VFS: Unable to mount root fs|cannot mount root|Cannot open root device",
    re.IGNORECASE,
)
ANSI_PATTERN = re.compile(r"\x1b\[[0-9;]*m")


class BootLog:
    def __init__(self, process):
        self.process = process
        self.lines = queue.Queue()
        self.history = []
        self.reader = threading.Thread(target=self._read_output, daemon=True)
        self.reader.start()

    def _read_output(self):
        for line in self.process.stdout:
            self.lines.put(line)
        self.lines.put(None)

    def next_line(self, timeout):
        try:
            line = self.lines.get(timeout=timeout)
        except queue.Empty:
            if self.process.poll() is not None:
                raise RuntimeError(
                    f"QEMU exited early with status {self.process.returncode}"
                )
            return ""
        if line is None:
            raise RuntimeError(
                f"QEMU output closed with status {self.process.poll()}"
            )
        self.history.append(line)
        print(line, end="", flush=True)
        if "PROTEA_DESKTOP_SIGN_IN_FOCUS_FAILED" in line:
            raise RuntimeError("The lock-screen Sign In button could not receive focus")
        if CRASH_PATTERN.search(ANSI_PATTERN.sub("", line)):
            raise RuntimeError(f"Guest reported a crash or boot failure: {line.strip()}")
        return line

    def wait_for(self, marker, timeout):
        if any(marker in line for line in self.history):
            return
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            line = self.next_line(min(0.25, deadline - time.monotonic()))
            if marker in line:
                return
        raise TimeoutError(f"Timed out waiting for {marker}")

    def wait_for_count(self, marker, count, timeout):
        deadline = time.monotonic() + timeout
        found = sum(marker in line for line in self.history)
        while found < count and time.monotonic() < deadline:
            line = self.next_line(min(0.25, deadline - time.monotonic()))
            found += marker in line
        if found < count:
            raise TimeoutError(f"Received {found}/{count} {marker} markers")


def qmp_command(qmp, reader, command, arguments=None):
    request = {"execute": command}
    if arguments is not None:
        request["arguments"] = arguments
    qmp.sendall((json.dumps(request) + "\r\n").encode("ascii"))
    while True:
        line = reader.readline()
        if not line:
            raise RuntimeError("QEMU monitor connection closed")
        response = json.loads(line)
        if "event" in response:
            continue
        if "error" in response:
            raise RuntimeError(f"QEMU monitor rejected {command}: {response['error']}")
        if "return" in response:
            return response["return"]


def send_key(qmp, reader, key):
    qmp_command(
        qmp,
        reader,
        "send-key",
        {
            "keys": [{"type": "qcode", "data": key}],
            "hold-time": 100,
        },
    )


def connect_qmp(path, process):
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        if process.poll() is not None:
            raise RuntimeError(f"QEMU exited before the monitor was ready ({process.returncode})")
        qmp = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        try:
            qmp.connect(str(path))
            reader = qmp.makefile("r", encoding="utf-8")
            greeting = json.loads(reader.readline())
            if "QMP" not in greeting:
                raise RuntimeError("QEMU monitor did not send a QMP greeting")
            qmp_command(qmp, reader, "qmp_capabilities")
            return qmp, reader
        except (FileNotFoundError, ConnectionRefusedError):
            qmp.close()
            time.sleep(0.1)
        except Exception:
            qmp.close()
            raise
    raise TimeoutError("Timed out connecting to the QEMU monitor")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--kernel", required=True, type=Path)
    parser.add_argument("--rootfs", required=True, type=Path)
    parser.add_argument(
        "--qemu", default=shutil.which("qemu-system-x86_64"), help="QEMU executable"
    )
    args = parser.parse_args()

    if not args.qemu:
        parser.error("qemu-system-x86_64 was not found; pass --qemu")
    for image in (args.kernel, args.rootfs):
        if not image.is_file():
            parser.error(f"image does not exist: {image}")

    with tempfile.TemporaryDirectory(prefix="protea-qemu-") as temp_dir:
        qmp_path = Path(temp_dir) / "monitor.sock"
        command = [
            args.qemu,
            "-m",
            "1024",
            "-smp",
            "2",
        ]
        if os.access("/dev/kvm", os.W_OK):
            command.extend(["-enable-kvm", "-cpu", "host"])
        command.extend(
            [
                "-kernel",
                str(args.kernel.resolve()),
                "-drive",
                f"file={args.rootfs.resolve()},format=raw,if=virtio",
                "-device",
                "virtio-gpu-pci",
                "-netdev",
                "user,id=net0",
                "-device",
                "virtio-net-pci,netdev=net0",
                "-append",
                "root=/dev/vda console=ttyS0 protea.qemu_test=1",
                "-qmp",
                f"unix:{qmp_path},server=on,wait=off",
                "-nographic",
            ]
        )

        process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
        )
        qmp = None
        reader = None
        try:
            log = BootLog(process)
            qmp, reader = connect_qmp(qmp_path, process)
            log.wait_for("PROTEA_BOOT_OK", 60)
            log.wait_for("PROTEA_GRAPHICS_OK", 15)
            log.wait_for("PROTEA_DESKTOP_WINDOW_PRESENTED", 30)
            log.wait_for("PROTEA_DESKTOP_TEST_MODE", 10)
            log.wait_for("PROTEA_DESKTOP_LOCK_READY", 10)
            log.wait_for("PROTEA_DESKTOP_SIGN_IN_FOCUSED", 1)

            time.sleep(0.6)
            send_key(qmp, reader, "ret")
            log.wait_for("PROTEA_DESKTOP_SIGNED_IN", 10)
            log.wait_for_count("PROTEA_DESKTOP_HEARTBEAT", 2, 10)

            if process.poll() is not None:
                raise RuntimeError(f"QEMU exited after sign-in ({process.returncode})")
            print("PROTEA_QEMU_INTERACTIVE_TEST_OK")
            return 0
        except (RuntimeError, TimeoutError) as error:
            print(f"Interactive QEMU test failed: {error}", file=sys.stderr)
            return 1
        finally:
            if reader is not None:
                reader.close()
            if qmp is not None:
                qmp.close()
            if process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()


if __name__ == "__main__":
    sys.exit(main())
