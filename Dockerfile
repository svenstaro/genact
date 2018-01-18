FROM rust:1.23.0-jessie

ENV APP_HOME /usr/src/app

RUN mkdir -p $APP_HOME
WORKDIR $APP_HOME

ADD . $APP_HOME

RUN ["cargo", "build", "--release"]

CMD ["./target/release/genact"]
