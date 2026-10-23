#!/usr/bin/python

import os
import pathlib
import subprocess
import sys


class Cargo:
    class Target:
        x86_64 = "x86_64-unknown-linux-gnu"
        aarch64 = "aarch64-unknown-linux-gnu"
        host = None

    def build(target: Target, is_release: bool):
        if is_release:
            if target == Cargo.Target.host:
                subprocess.run(["cargo", "build", "--release"])
            else:
                subprocess.run(["cargo", "build", "--target", target, "--release"])
        else:
            if target == Cargo.Target.host:
                subprocess.run(["cargo", "build"])
            else:
                subprocess.run(["cargo", "build", "--target", target])

    def determine_target(arch) -> Target:
        if arch == OutputPackage.Arch.aarch64:
            target = Cargo.Target.aarch64
        elif arch == OutputPackage.Arch.x86_64:
            target = Cargo.Target.x86_64
        return target


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
        f"    --version=[VERSION]     Version to package to\n\n"
        f"    --type=TYPE[,TYPE...]   Target packages [all, spf, rpm, deb, bin]\n"
        f"    --arch=ARCH[,ARCH...]   Target architectures [all, x86_64, aarch64]\n\n"
        f"Example:\n"
        f"    {method_to_exec} --type=deb,spf --arch=all --version=v0.8.0"
    )


def determine_spf_metadata(version: str, target: str, arch: str) -> str:

    return f"""
:::META DEFINE START:::
PROJECT_NAME = spf
VERSION = {version}
DESCRIPTION = Simple package manager
REPOSITORY = https://github.com/owen-debiasio/spf
LICENSE = gplv3
AUTHORS = Owen DeBiasio
ARCH = {arch}
:::META DEFINE END:::

:::PATH DEFINE START:::
target/{target}/release/spf:/usr/bin/spf
changes.md:/usr/share/doc/spf/changes.md
README.md:/usr/share/doc/spf/README.md
LICENSE:/usr/share/licenses/spf/LICENSE
:::PATH DEFINE END:::
"""


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

    def assemble(version, package_type: OutputPackage.Type, arch: OutputPackage.Arch):
        type_is_str = type(package_type).__name__ == "str"
        arch_is_str = type(arch).__name__ == "str"

        if (
            type_is_str
            and arch_is_str
            and not package_type is OutputPackage.Type.all
            and not arch is OutputPackage.Arch.all
        ):
            match package_type:
                case OutputPackage.Type.bin:
                    extension = ""
                case OutputPackage.Type.deb:
                    extension = f".{OutputPackage.Type.deb}"
                case OutputPackage.Type.rpm:
                    extension = f".{OutputPackage.Type.rpm}"
                case OutputPackage.Type.spf:
                    extension = f".{OutputPackage.Type.spf}"

            file_name = f"./packages/artifacts/spf-{version}-{arch}-linux{extension}"

            target = Cargo.determine_target(arch)
            Cargo.build(target, is_release=True)

            print("> Building...\n")

            match package_type:
                case OutputPackage.Type.spf:
                    Cargo.build(Cargo.Target.host, is_release=False)

                    with open("spfbuildcfg", "a") as meta:
                        meta.write(determine_spf_metadata(version, target, arch))
                    subprocess.run(
                        [
                            f"./target/debug/spf",
                            "create",
                            "spfbuildcfg",
                            file_name,
                        ]
                    )
                    os.remove("spfbuildcfg")
                case OutputPackage.Type.deb:
                    subprocess.run(["cargo-deb", "--target", target, "-o", file_name])

                case OutputPackage.Type.rpm:
                    subprocess.run(
                        ["cargo-generate-rpm", "--target", target, "-o", file_name]
                    )

        else:
            error("Not implemented")


class BuildInit:
    def determine_version(retrieved_flag: str) -> str:

        collected_version = retrieved_flag.split("=")[1]

        version_split = collected_version.split(".")

        if not len(version_split) == 3:
            error(f"Version is formatted incorrectly: {version_split}", 1)

        major = version_split[0]
        minor = version_split[1]
        micro = version_split[2]

        if (
            not minor.isnumeric()
            or not micro.isnumeric()
            or not major.replace("v", "").isnumeric()
        ):
            error(
                f"Version {version_split} must not contain letters/symbols (excluding 'v')",
                1,
            )

        if major.startswith("v"):
            return collected_version
        else:
            return f"v{collected_version}"

    def determine_output_type(retrieved_flag: str) -> str:

        collected_type = retrieved_flag.split("=")[1]

        if OutputPackage.Type.all in collected_type:
            return OutputPackage.Type.all

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

        if OutputPackage.Arch.all in collected_type:
            return OutputPackage.Arch.all

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

    version_pos = next(
        (i for i, x in enumerate(ARGS) if x.startswith("--version=")), None
    )
    type_pos = next((i for i, x in enumerate(ARGS) if x.startswith("--type=")), None)
    arch_pos = next((i for i, x in enumerate(ARGS) if x.startswith("--arch=")), None)

    try:
        version = BuildInit.determine_version(ARGS[version_pos])
    except:
        error("Please provide a version for the output package(s).", 1)

    try:
        output_types = BuildInit.determine_output_type(ARGS[type_pos])
    except:
        output_types = "all"

    try:
        output_archs = BuildInit.determine_arch(ARGS[arch_pos])
    except:
        output_archs = "all"

    print(f"> Packaging to version: {version}")
    print(f"> Packaging to format(s): {output_types}")
    print(f"> Packaging with architecture(s): {output_archs}")

    subprocess.call("./dev/format.bash")
    subprocess.call("./packages/clean.bash")

    create_artifact_dir()

    OutputPackage.assemble(version, output_types, output_archs)

    print("Packaging complete!")


if __name__ == "__main__":
    main()
