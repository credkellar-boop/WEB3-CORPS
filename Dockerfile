# --- Stage 1: Build & Compilation ---
FROM rust:1.75-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# --- Stage 2: Final Secure Workspace Runtime ---
FROM debian:bookworm-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/web3-corps .
COPY --from=builder /app/config/settings.json ./config/

RUN useradd -m appuser && chown -R appuser /usr/local/bin
USER appuser

ENV RUST_LOG=info
CMD ["./web3-corps"]
