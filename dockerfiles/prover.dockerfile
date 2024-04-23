# Use Rust with Alpine Linux as the base image for all stages
FROM rust:1.77.2-alpine AS base
# Install cargo-chef and build dependencies
RUN apk add --no-cache musl-dev openssl-dev && \
    cargo install cargo-chef
WORKDIR /app

# Create a plan stage that copies all files and creates a recipe.json if it does not exist
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Use the base stage to install dependencies as per the generated recipe.json
FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# Now, copy all files and build the actual application
COPY . .
RUN cargo build --release --package prover

# Create the runtime stage without the full Rust environment
FROM alpine AS runtime
# Install runtime dependencies if there are any, often libraries like ca-certificates, openssl, etc.
RUN apk add --no-cache libgcc
# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/prover /usr/local/bin/prover
ENTRYPOINT ["/usr/local/bin/prover"]
