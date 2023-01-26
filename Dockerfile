FROM docker.io/rust:1.66 as builder
WORKDIR /usr/src/smtp-proxy
COPY . .
RUN cargo install --path . --root .

FROM debian:buster-slim
ENV PORT=25
COPY --from=builder /usr/src/smtp-proxy/bin/smtp-proxy /usr/local/bin/smtp-proxy
CMD ["smtp-proxy"]