LABEL authors="arteii"

# Use a Rust base image
FROM rust:latest as builder


# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock to optimize caching for dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty dummy project to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the entire source code to the container
COPY . .

RUN cargo build --release

FROM rust:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/redirect /app/redirect

COPY redirect.conf /app/redirect.conf

EXPOSE 8000

CMD ["./redirect"]
