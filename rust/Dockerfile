# Rustバージョンの指定
ARG RUST_VERSION=1.71.0
ARG DEBAIN_VERSION=11

# 開発用ステージ
FROM rust:$RUST_VERSION AS build-env
WORKDIR /app
COPY ./api /app
RUN cargo build --release

# 本番用ステージ
FROM gcr.io/distroless/cc-debian$DEBAIN_VERSION AS production
COPY --from=build-env /app/target/release/api /
CMD ["./api"]