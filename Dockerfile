FROM rust as builder

ENV APP_HOME /usr/src/app

RUN mkdir -p $APP_HOME
WORKDIR $APP_HOME

ADD . $APP_HOME

RUN ["cargo", "build", "--release"]

FROM bitnami/minideb
COPY --from=builder /usr/src/app/target/release/genact /app/
ENTRYPOINT ["/app/genact"]
