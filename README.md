# Kratix-based Flink Promise Operator

This project provides a Kubernetes operator for managing Flink jobs using a Kratix-backed state store.

[![Docker](https://github.com/opencredo/promise-flink/actions/workflows/docker-publish.yml/badge.svg?branch=main)](https://github.com/opencredo/promise-flink/actions/workflows/docker-publish.yml)

[![Super-Linter](https://github.com/opencredo/promise-flink/actions/workflows/lint.yml/badge.svg)](https://github.com/marketplace/actions/super-linter)

## Prerequisites

- A running Kubernetes cluster
- Flink Kubernetes Operator installed (See: [https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/](https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/))
- Docker environment with the ability to build images for both amd64 and arm64 architectures.

```bash
chmod 777 ./internal/configure-pipeline/tests/test-output
docker run -e RUST_BACKTRACE=1 \
  -e KRATIX_WORKFLOW_TYPE='promise' \
  -e KRATIX_MANIFEST='/kratix/input/object.yaml' \
  $PIPELINE_NAME
```

## MiniKube

### Build local pipeline image
```bash
minikube start --driver qemu --memory=4Gb 
./internal/scripts/pipeline-image build
```

### Kratix Logs (Debugging)
```bash 
kubectl --context $PLATFORM get crds flinkdep.marketplace.kratix.io
kubectl logs -l=kratix-promise-id=flinkdep -n kratix-platform-system

```

### Setup (Promise)
```bash
kubectl apply --context $PLATFORM --filename promise.yaml
```
```bash
kubectl --context $WORKER get pods --watch
```

### Teardown (Promise)
```bash
kubectl delete --context $PLATFORM --filename promise.yaml

```

### Setup (Request)
```bash
kubectl apply --context $PLATFORM --filename resource-request.yaml
```

### Teardown (Request)
```bash
kubectl delete --context $PLATFORM --filename resource-request.yaml
```