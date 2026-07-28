FROM rust:alpine AS build
WORKDIR /tmp/build

# FROM alpine
RUN apk update
RUN apk add sqlite sqlite-dev sqlite-libs sqlite-static
# WORKDIR /tmp/build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/main.rs
RUN cargo fetch --locked
COPY src/ /tmp/build/src/
RUN cargo build --locked --release
# RUN ls -alh ./target/release/teefs-admin-api
# RUN apk add file
# RUN file ./target/release/teefs-admin-api >&2 ; false

FROM scratch
COPY --from=build /tmp/build/target/release/teefs-admin-api /
CMD ["/teefs-admin-api"]

# FROM alpine
# COPY --from=build /tmp/build/target/release/teefs-admin-api /usr/local/bin/

# FROM ubuntu
# COPY --from=build /tmp/build/target/release/teefs-admin-api /usr/local/bin/
