# Use multi staging to build the app.
FROM rust:slim AS build

WORKDIR /app

COPY backend/src ./src
COPY backend/Cargo.toml .
COPY backend/Cargo.lock .


# Install native C dependencies if required by your crates (e.g., OpenSSL)
# RUN apt update && apt  install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

# use distroless + glibc
FROM gcr.io/distroless/cc-debian12

COPY --from=build /app/target/release/backend /backend

EXPOSE 8080
# Run as a non-privileged
USER nonroot:nonroot
CMD ["./backend"]
