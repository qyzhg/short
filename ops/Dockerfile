FROM rust as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

WORKDIR /usr/app
COPY --from=builder /usr/src/app/target/release/tiny-url /usr/app/tiny-url
COPY ./config/ /usr/app/config/
COPY ./dist/ /usr/app/dist/

EXPOSE 8000

CMD ["/usr/app/tiny-url"]
