FROM rust:1.76.0-slim-bookworm AS builder
RUN apt-get update && apt-get install -y libssl-dev ca-certificates curl
RUN curl -sL https://deb.nodesource.com/setup_20.x | bash - && apt-get install -y nodejs

WORKDIR /app
COPY . .


RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/invoices ./invoices

RUN npx tailwindcss -o ./static/app.css --minify

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates

RUN useradd -ms /bin/bash app
USER app
WORKDIR /app

COPY --from=builder /app/invoices /app/invoices
COPY --from=builder /app/static /app/static
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080

CMD ./invoices
