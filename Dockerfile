FROM rust:slim-stretch AS builder

WORKDIR /app

RUN mkdir src

COPY src/. src/.
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release

CMD ["./target/release/rust_guessing_game"]

FROM rust:slim-stretch

WORKDIR /app

COPY --from=builder /app/target/release/rust_guessing_game /app/.

CMD ["./rust_guessing_game"]
