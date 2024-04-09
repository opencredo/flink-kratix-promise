FROM rust:latest AS rust-build
WORKDIR /app

COPY ./driver ./

RUN cargo build --release

# Stage 1: Build Stage
FROM alpine:3.19 AS build

# Create a non-root user
RUN adduser -D kratix

LABEL org.opencontainers.image.authors "info@opencredo.com"
LABEL org.opencontainers.image.source https://github.com/opencredo/promise-flink

RUN [ "mkdir", "/tmp/transfer" ]
RUN apk update && apk add --no-cache yq

COPY resources /tmp/transfer/resources/
COPY dependencies/ /tmp/transfer/dependencies/
COPY ./execute-pipeline execute-pipeline

RUN chmod +x execute-pipeline &&\
        ./execute-pipeline build

FROM alpine:3.19 
WORKDIR /app
USER kratix 
COPY --from=rust-build /app/target/release/driver /app/ 
ENTRYPOINT ["./app/driver execute-pipeline"]