FROM scratch
COPY --chmod=755 genact /app/
ENTRYPOINT ["/app/genact"]
