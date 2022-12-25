FROM rust:1-alpine as builder

WORKDIR /data/app
COPY . .

RUN apk add build-base
RUN rustup default nightly
RUN cargo build --release

# Runner container to avaid 2.5GB image sizes
FROM fedora:37

WORKDIR /data/app

COPY private private
COPY static static
COPY Rocket.toml .
COPY --from=builder /data/app/target/release/rust-text-converter /usr/local/bin/rust-text-converter

EXPOSE 8000/tcp
CMD [ "/usr/local/bin/rust-text-converter" ]
