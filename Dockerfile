FROM rust as builder

ENV APP_HOME /usr/src/app/

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y upx

COPY . $APP_HOME
WORKDIR $APP_HOME
RUN make build-linux

FROM scratch
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/genact /app/
ENTRYPOINT ["/app/genact"]
