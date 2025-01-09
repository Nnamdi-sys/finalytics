# Use the Rust builder image
FROM rust:1.81.0-slim

# Set the working directory
WORKDIR /app

# Copy the project files to the working directory
COPY . .

# Install necessary dependencies for SQLite3, OpenSSL, and Python
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    perl \
    cpanminus \
    make

# Install libipc-cmd-perl
RUN cpanm IPC::Cmd

# Build only the finalytics-web crate with SQLite3 support
RUN cargo build --release --package finalytics-web

# Copy the necessary directories to the right location
COPY /web/src/components /app/src/components
COPY /web/src/images /app/src/images
COPY /web/src/templates /app/src/templates

EXPOSE 8080

# Define the command to run your application
CMD ["/app/target/release/finalytics-server"]