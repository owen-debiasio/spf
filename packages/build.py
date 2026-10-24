#!/usr/bin/env python3

import os
import pathlib
import platform
import shutil
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
                subprocess.run(["cargo", "build", "--release"], check=False)
            else:
                subprocess.run(
                    ["cargo", "build", "--target", target, "--release"], check=False
                )
        else:
            if target == Cargo.Target.host:
                subprocess.run(["cargo", "build"], check=False)
            else:
                subprocess.run(["cargo", "build", "--target", target], check=False)

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
        f"spf Packager v0.1.2\n\n"
        f"Usage: {method_to_exec} [OPTIONS]\n\n"
        f"Options:\n"
        f"    --version=[VERSION]     Version to package to [crate, vX.X.X]\n"
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

        valid_types = {spf, deb, rpm, bin, all, ""}  # noqa: RUF012

    class Arch:
        x86_64 = "x86_64"
        aarch64 = "aarch64"
        all = "all"

        valid_types = {x86_64, aarch64, all, ""}  # noqa: RUF012

    def get_extension(type_to_convert) -> str:
        match type_to_convert:
            case OutputPackage.Type.bin:
                return ""
            case OutputPackage.Type.deb:
                return f".{OutputPackage.Type.deb}"
            case OutputPackage.Type.rpm:
                return f".{OutputPackage.Type.rpm}"
            case OutputPackage.Type.spf:
                return f".{OutputPackage.Type.spf}"

    def build_spf(pack_version, pack_target, pack_arch, dest):
        Cargo.build(Cargo.Target.host, is_release=False)

        with open("spfbuildcfg", "a") as meta:
            meta.write(determine_spf_metadata(pack_version, pack_target, pack_arch))
        subprocess.run(
            [
                "./target/debug/spf",
                "create",
                "spfbuildcfg",
                dest,
            ],
            check=False,
        )
        os.remove("spfbuildcfg")

    def build_deb(pack_t, pack_dest):
        subprocess.run(["cargo-deb", "--target", pack_t, "-o", pack_dest], check=False)

    def build_rpm(pack_t, pack_dest):
        subprocess.run(
            ["cargo-generate-rpm", "--target", pack_t, "-o", pack_dest], check=False
        )

    def build_all_types(package_type, version, target, arch, dest):
        match package_type:
            case OutputPackage.Type.spf:
                OutputPackage.build_spf(version, target, arch, dest)
            case OutputPackage.Type.deb:
                OutputPackage.build_deb(target, dest)
            case OutputPackage.Type.rpm:
                OutputPackage.build_rpm(target, dest)
            case OutputPackage.Type.bin:
                shutil.copy2(f"target/{target}/release/spf", dest)

    def build_and_package(version, package_type, arch):
        """Helper function to compile and package a specific variant."""
        file_name = (
            f"spf-{version}-{arch}-linux{OutputPackage.get_extension(package_type)}"
        )
        dest = f"./packages/artifacts/{file_name}"

        target = Cargo.determine_target(arch)
        Cargo.build(target, is_release=True)
        OutputPackage.build_all_types(package_type, version, target, arch, dest)

    def assemble(version, package_types, archs):
        print("> Building...\n")

        all_packages = [
            OutputPackage.Type.spf,
            OutputPackage.Type.deb,
            OutputPackage.Type.rpm,
            OutputPackage.Type.bin,
        ]
        all_archs = [OutputPackage.Arch.x86_64, OutputPackage.Arch.aarch64]

        if isinstance(package_types, list):
            if OutputPackage.Type.all in package_types:
                target_types = all_packages
            else:
                target_types = package_types
        elif package_types == OutputPackage.Type.all:
            target_types = all_packages
        else:
            target_types = [package_types]

        if isinstance(archs, list):
            if OutputPackage.Arch.all in archs:
                target_archs = all_archs
            else:
                target_archs = archs
        elif archs == OutputPackage.Arch.all:
            target_archs = all_archs
        else:
            target_archs = [archs]

        for current_arch in target_archs:
            for current_type in target_types:
                OutputPackage.build_and_package(version, current_type, current_arch)


class BuildInit:
    def determine_version(retrieved_flag: str) -> str:

        collected_version = retrieved_flag.split("=")[1]

        if collected_version == "crate" or len(collected_version) == 0:
            crate_version = subprocess.run(
                'cargo pkgid | cut -d "#" -f2',
                check=False,
                shell=True,
                capture_output=True,
                text=True,
            )

            return "v" + crate_version.stdout.removesuffix("\n")

        version_split = collected_version.split(".")

        if len(version_split) != 3:
            error(f"Version is formatted incorrectly: {version_split}", 1)

        major = version_split[0]
        minor = version_split[1]
        micro = version_split[2]

        if (
            not minor.isnumeric()
            or not micro.isnumeric()
            or not major.removeprefix("v").isnumeric()
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

            if len(collected_type) == len(OutputPackage.Type.valid_types) - 2:
                return OutputPackage.Type.all

        if type(collected_type).__name__ == "list":
            filtered_list = list(set(collected_type) - OutputPackage.Type.valid_types)

            if len(filtered_list) != 0:
                error(f"Invalid package type(s): {filtered_list}", 1)
            else:
                return collected_type
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
                        return OutputPackage.Type.all
                    else:
                        return collected_type

                case _:
                    error(f'Invalid output type: "{collected_type}".', 1)

    def determine_arch(retrieved_flag: str) -> str:

        collected_arch = retrieved_flag.split("=")[1]

        if OutputPackage.Arch.all in collected_arch:
            return OutputPackage.Arch.all

        if "," in collected_arch:
            collected_arch = collected_arch.removesuffix(",").split(",")

            if len(collected_arch) == len(OutputPackage.Arch.valid_types) - 2:
                return OutputPackage.Arch.all

        if type(collected_arch).__name__ == "list":
            filtered_list = list(set(collected_arch) - OutputPackage.Arch.valid_types)

            if len(filtered_list) != 0:
                error(f"Invalid arch(s): {filtered_list}", 1)
            else:
                return collected_arch
        else:
            match collected_arch:
                case (
                    OutputPackage.Arch.x86_64
                    | OutputPackage.Arch.aarch64
                    | OutputPackage.Arch.all
                    | ""
                ):
                    if collected_arch == OutputPackage.Arch.all or collected_arch == "":
                        return OutputPackage.Arch.all
                    else:
                        return collected_arch

                case _:
                    error(f'Invalid output arch: "{collected_arch}".', 1)


def main():

    if platform.system() == "Windows":
        input("WARNING: I encourage using WSL when developing spf!\n")

    arg_len = len(ARGS)

    if arg_len < 2:
        intro()

        sys.exit(1)

    def find_flag(flag):
        return next((i for i, x in enumerate(ARGS) if x.startswith(f"--{flag}=")), None)

    version_pos = find_flag("version")
    type_pos = find_flag("type")
    arch_pos = find_flag("arch")

    try:
        version = BuildInit.determine_version(ARGS[version_pos])
    except:  # noqa: E722
        error("Please provide a version for the output package(s).", 1)

    try:
        retrieved_types = ARGS[type_pos]
    except:  # noqa: E722
        retrieved_types = "--type="

    try:
        retrieved_archs = ARGS[arch_pos]
    except:  # noqa: E722
        retrieved_archs = "--arch="

    output_types = BuildInit.determine_output_type(retrieved_types)
    output_archs = BuildInit.determine_arch(retrieved_archs)

    print(f"> Packaging to version: {version}")
    print(f"> Packaging to format(s): {output_types}")
    print(f"> Packaging with architecture(s): {output_archs}")

    subprocess.call("./dev/format.bash")
    subprocess.call("./packages/clean.bash")

    create_artifact_dir()

    OutputPackage.assemble(version, output_types, output_archs)

    print("\n> Packaging complete!")


if __name__ == "__main__":
    main()
