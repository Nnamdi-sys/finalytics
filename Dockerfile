# Use the Rust builder image
FROM rust:slim-buster AS builder

# Set the working directory
WORKDIR /src

# Copy the local finalytics directory into the Docker image
COPY finalytics finalytics

# Copy the local web directory into the Docker image
COPY web web

# Copy the pre-generated symbols.db file into the web directory
COPY web/symbols.db web/symbols.db

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
RUN dx build --release --features web && cargo build --release --features server

# Create the final runtime image
FROM fedora:34 AS runner

# Install necessary dependencies for SQLite3, OpenSSL, and other required tools
RUN dnf install -y pkgconfig openssl-devel sqlite-devel

# Set the working directory to /web
WORKDIR /web

# Copy the application binary
COPY --from=builder /src/web/dist /web/dist
COPY --from=builder /src/web/target/release/finalytics-web /web/dist

EXPOSE 8080

# Define the command to run your application
CMD ["/web/dist/finalytics-web"]