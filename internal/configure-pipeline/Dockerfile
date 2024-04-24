FROM rust:1.77.2-slim-buster as build-env

LABEL org.opencontainers.image.authors "info@opencredo.com"
LABEL org.opencontainers.image.source https://github.com/opencredo/promise-flink


WORKDIR /app

RUN apt-get update && apt-get install -y librust-openssl-dev libssl-dev

RUN set -eux; \
	adduser -u 70 -S -D -G kratix -H -h /app -s /bin/sh kratix; \
	mkdir -p /app; \
	mkdir -p /kratix; \
	chown 775 /app; \
	chown 775 /kratix;

COPY . /app

RUN mkdir /tmp/transfer \
    rm -rf /app/target \
	cargo build --release;

FROM gcr.io/distroless/cc-debian12

ENV RUST_LOG=debug
ENV KRATIX_WORKFLOW_TYPE="promise"
ENV PIPELINE_NAME="opencredo/flink-configure-pipeline:dev"
ENV BASE_INSTANCE="/tmp/transfer/resources/minimal-flinkdep-manifest.yaml"
ENV KRATIX_OUTPUT="/kratix/output"
ENV KRATIX_INPUT="/kratix/input"
ENV DEPENDENCIES_DIR="/tmp/transfer/dependencies/"
ENV RESOURCES_DIR="/tmp/transfer/resources/"


COPY --from=build-env /app/resources $RESOURCES_DIR
COPY --from=build-env /app/dependencies/ $DEPENDENCIES_DIR

COPY --from=build-env /app/target/release/configure-pipeline /
COPY --from=build-env /app/tests/ /kratix

ENV RUST_BACKTRACE=1



CMD ["./configure-pipeline","pipeline"] 
ENTRYPOINT []