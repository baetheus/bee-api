FROM alpine as BUILD

RUN apk add rust cargo

WORKDIR /build
COPY src ./src
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --target x86_64-alpine-linux-musl --release

FROM alpine

COPY --from=BUILD /build/target/x86_64-alpine-linux-musl/release/bee-api /usr/local/bin/bee-api 

ENTRYPOINT [ "bee-api" ]