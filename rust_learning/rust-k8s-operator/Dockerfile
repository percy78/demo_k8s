# Build stage
FROM rust:1.75-bookworm as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install CA certificates and necessary libraries
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -u 1000 -m operator

# Copy the binary from builder stage
COPY --from=builder /app/target/release/k8s-operator /usr/local/bin/k8s-operator

# Change ownership and make executable
RUN chown operator:operator /usr/local/bin/k8s-operator && \
    chmod +x /usr/local/bin/k8s-operator

# Switch to non-root user
USER operator

# Set the startup command
ENTRYPOINT ["/usr/local/bin/k8s-operator"]