
FROM debian:bullseye-slim AS builder

#toolchains
RUN apt-get update && \
    apt-get install -y curl build-essential ca-certificates pkg-config libssl-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup default stable

WORKDIR /app

#copy project
COPY . .
RUN . "$HOME/.cargo/env" && cargo build --release

FROM debian:bullseye-slim

# runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

#copy the built binaries and res folder
COPY --from=builder /app/target/release/server .
COPY --from=builder /app/res ./res

EXPOSE 1111 

CMD ["sh", "-c", "ls && ./server"]
