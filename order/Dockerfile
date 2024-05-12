FROM rust:1.75.0-slim-bullseye AS build

WORKDIR /build

COPY . .

RUN cargo build

ENTRYPOINT ["cargo", "run"]




# FROM rust:alpine3.14 AS build
#
# WORKDIR /build
#
# COPY Cargo.lock Cargo.toml ./
# RUN mkdir src \
#     && echo "// dummy file" > src/lib.rs \
#     && cargo build --release
#
# COPY src src
# RUN cargo build --locked --release
# RUN cp ./target/release/$ORDER_APP_NAME /bin/server
#
# FROM alpine:3.14
#
# WORKDIR /usr/local/bin
# COPY --from=build /bin/server .
# # COPY --from target/release/order .
# ENTRYPOINT "/usr/local/bin/order"
