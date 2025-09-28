# Stage 1: Build & cache dependencies
FROM rust:bullseye as builder
WORKDIR /usr/src/app

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features sqlite

# Only copy files required for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy project to build and cache dependencies
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY ./src ./src
COPY migrations ./migrations
COPY diesel.toml .

# Build the application
RUN rm -f target/release/deps/contacts_api*
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bullseye-slim
# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled application and diesel CLI from the builder stage
COPY --from=builder /usr/src/app/target/release/contacts-api .
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/

# Copy and set up the entrypoint script
COPY entrypoint.sh .
RUN chmod +x entrypoint.sh

EXPOSE 8081

CMD ["./entrypoint.sh"]
