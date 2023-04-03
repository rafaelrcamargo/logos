# Using official rust base image
FROM rust:alpine

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

WORKDIR /app

COPY Cargo.toml ./

ARG SERVICE
RUN echo "Building service: $SERVICE"
RUN ls -la

COPY ./services/utils ./services/utils
COPY ./services/${SERVICE} ./services/${SERVICE}

EXPOSE 8000

ENTRYPOINT ["cargo", "watch", "-q", "-c", "-x"]
CMD ["run", "--bin", "${SERVICE}"]
