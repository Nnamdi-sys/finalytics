# Use the Rust builder image
FROM rust:slim-buster AS builder

# Set the working directory
WORKDIR /src

# Copy the local finalytics directory into the Docker image
COPY finalytics finalytics

# Copy the local apps directory into the Docker image
COPY apps apps

# Install necessary dependencies for SQLite3 and OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev libsqlite3-dev

# Install Perl and cpanminus, and make
RUN apt-get install -y perl cpanminus make

# Install libipc-cmd-perl
RUN cpanm IPC::Cmd

# Install Dioxus CLI and add wasm32-unknown-unknown target
RUN cargo install dioxus-cli --locked && rustup target add wasm32-unknown-unknown

# Change the working directory to the apps directory
WORKDIR /src/apps

# Build your Rust project with Dioxus CLI
RUN dx build --release --features web && cargo build --release --features server

# Create the final runtime image
FROM fedora:34 AS runner

# Install necessary dependencies for SQLite3, OpenSSL, and other required tools
RUN dnf install -y pkgconfig openssl-devel sqlite-devel

# Install Chrome (or Chromium) in the runner
RUN dnf install -y chromium

# Set the working directory to /apps
WORKDIR /apps

# Copy the application binary
COPY --from=builder /src/apps/dist /apps/dist
COPY --from=builder /src/apps/target/release/finalytics-apps /apps/dist

EXPOSE 8080
EXPOSE 8443
EXPOSE 9222

# Define the command to run your application
CMD ["/apps/dist/finalytics-apps"]