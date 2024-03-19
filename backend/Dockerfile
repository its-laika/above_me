FROM rust:1.76

WORKDIR /app

COPY src src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo install --path .

CMD ["above_me"]