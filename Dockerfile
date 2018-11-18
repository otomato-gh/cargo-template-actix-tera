from rust:slim as build
RUN USER=root cargo new --bin builddir
workdir /builddir

add Cargo.* /builddir/
run cargo build --release && rm src/*.rs
COPY ./src ./src
COPY ./templates ./templates
o
# buildme}} for release
RUN cargo clean && cargo build --release

from bitnami/minideb:stretch
copy --from=build /builddir/target/release/{{project-name}}/app
cmd /app
