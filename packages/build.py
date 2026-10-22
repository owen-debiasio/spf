#!/usr/bin/python

import os
import pathlib
import subprocess
import sys


class Cargo:
    class Target:
        x86_64 = "x86_64-unknown-linux-gnu"
        aarch64 = "aarch64-unknown-linux-gnu"

    def build(target: Target, is_release: bool):
        if is_release:
            subprocess.run(["cargo", "build", "--target", target, "--release"])
        else:
            subprocess.run(["cargo", "build", "--target", target])


def create_artifact_dir():
    current_dir = pathlib.Path(__file__).parent.resolve()
    artifact_folder = f"{current_dir}/artifacts"

    if not os.path.exists(artifact_folder):
        os.mkdir(artifact_folder)

ARGS = sys.argv

def intro():
    print(
        "spf Packager v0.1.0\n" \
        "\n" \
        "Usage:\n" \
        f"    {ARGS[0]} <--type=[spf, rpm, deb, bin]> <--arch=[all, x86_64, aarch64]>"
    )


def main():

    arg_len =len(ARGS)

    if arg_len < 2:
        intro()
        sys.exit(1)

    if arg_len == 1 and ARGS[1].startswith("--type="):
        print("Error: Missing version")

    subprocess.call("./dev/format.bash")
    subprocess.call("./packages/clean.bash")

    create_artifact_dir()

    Cargo.build(Cargo.Target.x86_64, True)


if __name__ == "__main__":
    main()
