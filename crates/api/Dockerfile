FROM ekidd/rust-musl-builder:latest AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin crabtyper-api

FROM alpine AS runtime
ENV DATABASE_URL=/var/lib/snippets.db
ENV PORT=5000
EXPOSE 5000
COPY --from=builder /app/snippets.db /var/lib/snippets.db
 COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/crabtyper-api /usr/local/bin/
CMD ["/usr/local/bin/crabtyper-api"]
