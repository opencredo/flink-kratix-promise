# Kratix-based Flink Promise Operator

This project provides a Kubernetes operator for managing Flink jobs using a Kratix-backed state store.

[![Entrypoint](https://github.com/opencredo/promise-flink/actions/workflows/entrypoint.yml/badge.svg)](https://github.com/opencredo/promise-flink/actions/workflows/entrypoint.yml)


## Prerequisites

- A running Kubernetes cluster
- Kratix [see install guide](https://docs.kratix.io/main/guides/installing-kratix/single-cluster)
- Docker environment with the ability to build images for both amd64 or arm64 architectures.

### Test Changes
```bash

# INPUT tests/test-input/object.yaml
# OUTPUT tests/test-output/
# ENVs see internal/configure-pipeline/.env

export WORKSPACE="<path-to-repo>"  
export KRATIX_WORKFLOW_TYPE="promise" or "resource"
```
```bash
cd internal/configure-pipeline
cargo build
cargo test -- --test-threads=1 # 1 thread is required only otherwise it will fail due to file managment
```

### Setup

#### Creating the workflow image

First, build the requisite workflow docker image. Two are available, based on Rust and shell. Both are functionally equivalent.

```bash
# rust based pipeline
docker build --tag opencredo/flink-configure-pipeline:dev ./internal/configure-pipeline
```

```bash
# shell based pipeline
docker build --tag opencredo/flink-configure-pipeline:dev ./shell/configure-pipeline
```

Next, load the image into your docker environment. Using kind:

```bash
kind load docker-image opencredo/flink-configure-pipeline:dev --name platform
```


### Creating the promise
Apply the promise:

```bash
kubectl apply --context $PLATFORM --filename promise.yaml
```

Now wait until the flink operator is available in the worker cluster:
```bash
kubectl --context $WORKER get pods --watch
```

### Creating the flink promise request

Now you can fulfil a [resource-request](resource-request.yaml) as a Flink job:
```bash
kubectl apply --context $PLATFORM --filename resource-request.yaml
```

### Kratix Verification

```bash
kubectl --context $PLATFORM get crds flinkdeps.example.promise.syntasso.io
kubectl logs -l=kratix-promise-id=flinkdep -n kratix-platform-system -c flinkdep-promise-pipeline
```

### Teardown

Deleting the resource:
```bash
kubectl delete --context $PLATFORM --filename resource-request.yaml
```

Deleting the promise:
```bash
kubectl delete --context $PLATFORM --filename promise.yaml
```
