# Build stage
FROM rust:latest as build

WORKDIR /app

COPY . .

RUN cargo build --release

# Run stage
FROM debian:latest

COPY --from=build /app/target/release/strand .
RUN apt-get update && apt-get install -y ca-certificates openssl 
CMD ["./strand"]