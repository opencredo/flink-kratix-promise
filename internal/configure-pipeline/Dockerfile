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

RUN chmod +x execute-pipeline 

RUN ./execute-pipeline build

# Stage 2: Production Stage
FROM alpine:3.19

WORKDIR /app

# Switch to the non-root user
USER kratix

# Copy artifacts from the build stage
COPY --from=build /tmp/transfer /app

CMD [ "sh", "-c", "./execute-pipeline" ]
ENTRYPOINT []