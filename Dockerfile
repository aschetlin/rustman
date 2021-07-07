FROM jupyter/base-notebook

USER root
RUN apt-get update && apt-get -y upgrade
ENV RUSTUP_HOME /usr/local/rustup
ENV CARGO_HOME /usr/local/cargo
ENV PATH /usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        cmake \
        build-essential \
        ; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    \
    apt-get remove -y --auto-remove \
        wget \
        ; \
    rm -rf /var/lib/apt/lists/*;

USER 1000
RUN rustup component add rust-src
RUN cargo install --force --git https://github.com/google/evcxr.git evcxr_jupyter
RUN evcxr_jupyter --install
RUN cargo install evcxr_repl
USER root
RUN apt-get update && apt-get install -y libssl-dev pkg-config
USER 1000
RUN cargo install sccache
# WORKDIR /work

ENV JUPYTER_ENABLE_LAB yes
