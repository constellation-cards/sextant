FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src
COPY src/ src/
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl1.1
COPY --from=builder /app/target/release/sextant . 
EXPOSE 8000

CMD ["./sextant"]