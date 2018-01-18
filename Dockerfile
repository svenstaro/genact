FROM alpine:3.7

RUN wget https://github.com/svenstaro/genact/releases/download/0.2.2/genact-linux -O /tmp/genact-linux
RUN chmod +x /tmp/genact-linux
ENTRYPOINT ["/tmp/genact-linux"]