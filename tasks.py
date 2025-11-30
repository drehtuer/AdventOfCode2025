from invoke import context, task
import os

RUST_GDB = "rust-1.85-gdb"
RUST_FMT = "rust-1.85"
CARGO_BIN = "cargo-1.85"

SRC_DIR = "src"
MAIN_RS = "main.rs"
BIN_DIR = "bin"
BIN_FILE = "advent_of_code_2025"

CARGO_FLAGS = [
    "--color always",
    f"--target-dir {BIN_DIR}",
    "--manifest-path Cargo.toml",
]


def ctx_run(
    ctx: context,
    cmd: list[str],
) -> None:
    ctx.run(" ".join(cmd))


@task
def build_debug(ctx: context) -> None:
    os.makedirs(BIN_DIR, exist_ok=True)
    ctx_run(ctx, [CARGO_BIN, "build", " ".join(CARGO_FLAGS)])


@task
def build_release(ctx: context) -> None:
    os.makedirs(BIN_DIR, exist_ok=True)
    ctx_run(ctx, [CARGO_BIN, "build", " ".join(CARGO_FLAGS), "--release"])


@task
def clean(ctx: context) -> None:
    ctx_run(ctx, [CARGO_BIN, "clean", f"--target-dir {BIN_DIR}"])


@task
def run_debug(ctx: context) -> None:
    ctx_run(ctx, [os.path.join(BIN_DIR, "debug", BIN_FILE)])


@task
def run_release(ctx: context) -> None:
    ctx_run(ctx, [os.path.join(BIN_DIR, "release", BIN_FILE)])
