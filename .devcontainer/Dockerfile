# syntax=docker/dockerfile:1
# https://docs.docker.com/reference/dockerfile/
FROM  ubuntu:24.04

LABEL name="AdventOfCode2025 Dev Container" \
  maintainer="drehtuer" \
  version="1.0.0" \
  description="Development Container for AdventOfCode2025"

ARG   DEBIAN_FRONTEND=noninteractive
ARG   ARG_USER

ENV   LANG=en_US.utf-8 \
  LANGUAGE=en_US:en \
  LC_ALL=en_US.utf-8

RUN \
  # apt cache folders as tmpfs, discarded after build.
  # No need to delete these folders later
  --mount=type=tmpfs,target=/var/lib/apt/lists \
  --mount=type=tmpfs,target=/var/cache/apt \
  \
  apt-get update \
  && apt-get upgrade \
  --yes \
  -qq \
  && apt-get install \
  --no-install-recommends \
  --yes \
  -qq \
  bash \
  bash-completion \
  ca-certificates \
  curl \
  gcc \
  git \
  locales \
  luarocks \
  neovim \
  python-is-python3 \
  python3 \
  python3-invoke \
  ssh \
  sudo \
  # Generate en_US.UTF-8 locales
  && localedef \
  --inputfile=en_US \
  --force \
  --charmap=UTF-8 \
  --alias-file=/usr/share/locale/locale.alias \
  en_US.UTF-8 \
  # Add non-root user,
  # password needed for `sudo`
  && useradd \
  --create-home \
  --user-group \
  --groups \
  sudo \
  --shell /usr/bin/bash \
  ${ARG_USER} \
  && inv \
  --print-completion-script=bash \
  > /etc/bash_completion.d/invoke

COPY \
  --chown=root:root \
  --chmod=0700 \
  sudo.conf /etc/sudoers.d/sudoers

USER ${ARG_USER}

RUN \
  # Install rust
  curl \
  --proto '=https' \
  --tlsv1.2 \
  -sSf \
  https://sh.rustup.rs \
  | sh -s -- \
  -y \
  --default-toolchain stable

CMD {"sleep", "infinity"}
