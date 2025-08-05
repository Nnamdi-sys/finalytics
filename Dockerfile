# Use the Rust builder image
FROM rust:slim-bookworm AS builder

# Set the working directory
WORKDIR /src

# Copy the local finalytics directory into the Docker image
COPY rust rust

# Copy the local web directory into the Docker image
COPY web web

# Install necessary dependencies for SQLite3 and OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev libsqlite3-dev

# Install Perl and cpanminus, and make
RUN apt-get install -y perl cpanminus make

# Install libipc-cmd-perl
RUN cpanm IPC::Cmd

# Install Dioxus CLI and add wasm32-unknown-unknown target
RUN cargo install dioxus-cli --locked && rustup target add wasm32-unknown-unknown

# Change the working directory to the web directory
WORKDIR /src/web

# Build your Rust project with Dioxus CLI
RUN dx bundle --platform web

# Create the final runtime image
FROM debian:bookworm AS runner

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    openssl libssl3 ca-certificates libsqlite3-0

# Set the working directory to /web
WORKDIR /web

# Copy the application binary and frontend assets
COPY --from=builder /src/web/dist /web/dist
COPY --from=builder /src/web/target/release/finalytics-web /web/dist

# Set environment variable for production
ENV ENV=prod

EXPOSE 8080

# Define the command to run your application
CMD ["/web/dist/finalytics-web"]