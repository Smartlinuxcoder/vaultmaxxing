FROM rust:1.87.0-alpine3.22 AS chef
RUN apk add --no-cache build-base openssl-dev pkgconfig openssl-libs-static

RUN cargo install cargo-chef
WORKDIR /app

FROM oven/bun:alpine AS bun
COPY . .
RUN bun install
RUN bunx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
COPY --from=bun /home/bun/app/assets/tailwind.css ./assets/tailwind.css
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo install dioxus-cli --root /.cargo --force
ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --platform web

FROM chef AS runtime
COPY --from=builder /app/target/dx/vaultmaxxing/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]