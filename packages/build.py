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


def error(text, code: int):
    print(text)

    sys.exit(code)


def intro():
    method_to_exec = ARGS[0]
    print(
        f"spf Packager v0.1.0\n\n"
        f"Usage: {method_to_exec} [OPTIONS]\n\n"
        f"Options:\n"
        f"    --type=TYPE[,TYPE...]   Target packages [all, spf, rpm, deb, bin]\n"
        f"    --arch=ARCH[,ARCH...]   Target architectures [all, x86_64, aarch64]\n\n"
        f"Example:\n"
        f"    {method_to_exec} --type=all,spf --arch=all"
    )


class OutputPackage:
    class Type:
        spf = "spf"
        deb = "deb"
        rpm = "rpm"
        bin = "bin"
        all = "all"

        valid_types = {spf, deb, rpm, bin, all, ""}

    class Arch:
        x86_64 = "x86_64"
        aarch64 = "aarch64"
        all = "all"

        valid_types = {x86_64, aarch64, all, ""}


class BuildInit:
    def determine_output_type(retrieved_flag: str) -> str:

        collected_type = retrieved_flag.split("=")[1]

        if "," in collected_type:
            collected_type = collected_type.removesuffix(",").split(",")

        if type(collected_type).__name__ == "list":
            filtered_list = list(set(collected_type) - OutputPackage.Type.valid_types)

            if not len(filtered_list) == 0:
                error(f"Invalid package type(s): {filtered_list}", 1)
            else:
                output = collected_type
        else:
            match collected_type:
                case (
                    OutputPackage.Type.spf
                    | OutputPackage.Type.deb
                    | OutputPackage.Type.rpm
                    | OutputPackage.Type.bin
                    | OutputPackage.Type.all
                    | ""
                ):
                    if collected_type == OutputPackage.Type.all or collected_type == "":
                        output = OutputPackage.Type.all
                    else:
                        output = collected_type

                case _:
                    error(f'Invalid output type: "{collected_type}".', 1)

        return output

    def determine_arch(retrieved_flag: str) -> str:

        collected_type = retrieved_flag.split("=")[1]

        if "," in collected_type:
            collected_type = collected_type.removesuffix(",").split(",")

        if type(collected_type).__name__ == "list":
            filtered_list = list(set(collected_type) - OutputPackage.Arch.valid_types)

            if not len(filtered_list) == 0:
                error(f"Invalid arch(s): {filtered_list}", 1)
            else:
                output = collected_type
        else:
            match collected_type:
                case (
                    OutputPackage.Arch.x86_64
                    | OutputPackage.Arch.aarch64
                    | OutputPackage.Arch.all
                    | ""
                ):
                    if collected_type == OutputPackage.Arch.all or collected_type == "":
                        output = OutputPackage.Arch.all
                    else:
                        output = collected_type

                case _:
                    error(f'Invalid output arch: "{collected_type}".', 1)

        return output


def main():

    arg_len = len(ARGS)

    if arg_len < 2:
        intro()

        sys.exit(1)

    if arg_len < 3:
        error("Not enough arguments!", 1)

    if arg_len > 3:
        error("Too many arguments provided!", 1)

    output_types = BuildInit.determine_output_type(ARGS[1])
    output_archs = BuildInit.determine_arch(ARGS[2])

    print(f"> Packaging to format(s): {", ".join(output_types)}")
    print(f"> Packaging with architecture(s): {", ".join(output_archs)}")

    exit(0)

    subprocess.call("./dev/format.bash")
    subprocess.call("./packages/clean.bash")

    create_artifact_dir()

    Cargo.build(Cargo.Target.x86_64, is_release=True)


if __name__ == "__main__":
    main()
