FROM rust as builder
WORKDIR /usr/src
RUN USER=root cargo new tiny-url
WORKDIR /usr/src/tiny-url
RUN mkdir -p .cargo
COPY ./Cargo.toml Cargo.lock ./
COPY ./src src
RUN cargo install --path . --color always

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /usr/local/cargo/bin/tiny-url ./tiny-url
COPY ./config/ ./config/
COPY ./dist/ ./dist/
USER 1000
EXPOSE 8000
ENTRYPOINT ["./tiny-url"]