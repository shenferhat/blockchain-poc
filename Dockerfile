FROM rust:1.35.0
RUN rustup default nightly

WORKDIR /usr/src/avalanche
COPY . .

RUN cargo install --path .

CMD ["avalanche"]

EXPOSE 5001 5002 5003 5004 5005