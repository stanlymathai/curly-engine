## Build stage
FROM rust:1-slim-bullseye AS builder
WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build 
# downloading the index, which can take a long time within
# the docker context. Docker will cache it.

RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

# copy app files
COPY src src

## Build the app
RUN cargo build --release

## Production stage
FROM debian:bullseye-slim
WORKDIR /app

# Copy the build artifacts
COPY --from=builder /code/target/release/talkmoni_sso talkmoni_sso


## Set environment variables

# app
ARG APP_ENV
ENV APP_ENV ${APP_ENV}

ARG APP_KEY
ENV APP_KEY ${APP_KEY}

ARG APP_PORT
ENV APP_PORT ${APP_PORT}

ARG API_ENDPOINT
ENV API_ENDPOINT ${API_ENDPOINT}

# database
ARG DB_NAME
ENV DB_NAME ${DB_NAME}

ARG DB_HOST
ENV DB_HOST ${DB_HOST}

ARG DB_USERNAME
ENV DB_USERNAME ${DB_USERNAME}

ARG DB_PASSWORD
ENV DB_PASSWORD ${DB_PASSWORD}

ARG DB_SOCKET_TIMEOUT
ENV DB_SOCKET_TIMEOUT ${DB_SOCKET_TIMEOUT}

ARG DB_CONNECTION_TIMEOUT
ENV DB_CONNECTION_TIMEOUT ${DB_CONNECTION_TIMEOUT}


# Run as non-root user for security reasons 
RUN groupadd -r appuser && useradd -r -g appuser appuser
RUN chown -R appuser:appuser /app

# Switch to the appuser
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Run the app
CMD [ "/app/talkmoni_sso" ]