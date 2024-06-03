# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.74-bullseye as builder

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo build --release -vv

FROM debian:bullseye-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/leaky-server /app/

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
EXPOSE 3000

# Run the server
CMD ["/app/leaky-server"]

