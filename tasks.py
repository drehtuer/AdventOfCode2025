from invoke import context, task
import os

RUST_GDB = "rust-gdb"
RUST_FMT = "rust"
CARGO_BIN = "cargo"

BIN_DIR = "bin"

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
def run_debug(ctx: context, day: int) -> None:
    ctx_run(ctx, [os.path.join(BIN_DIR, "debug", f"aoc_25_day{day}")])


@task
def run_release(ctx: context, day: int) -> None:
    ctx_run(ctx, [os.path.join(BIN_DIR, "release", f"aoc_25_day{day}")])


@task
def debug(ctx: context, day: int) -> None:
    build_debug(ctx)
    run_debug(ctx, day)


@task
def release(ctx: context, day: int) -> None:
    build_release(ctx)
    run_release(ctx, day)
