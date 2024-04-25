# Stage 1: Build Stage
FROM alpine:3.19 AS build

LABEL org.opencontainers.image.authors "info@opencredo.com"
LABEL org.opencontainers.image.source https://github.com/opencredo/promise-flink

RUN [ "mkdir", "/tmp/transfer" ]
RUN [ "mkdir", "-p", "/kratix/output" ]
RUN apk update && apk add --no-cache yq

COPY resources /tmp/transfer/resources/
COPY dependencies/ /tmp/transfer/dependencies/
COPY execute-pipeline execute-pipeline

RUN chmod +x execute-pipeline

ENV KRATIX_WORKFLOW_TYPE="promise"
CMD [ "sh", "-c", "./execute-pipeline" ]
ENTRYPOINT []