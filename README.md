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

```bash
CLUSTER_NAME="kratix-labs"
minikube start --driver qemu --memory=4Gb --container-runtime=docker --nodes 1 -p $CLUSTER_NAME --cni=auto --addons=default-storageclass,registry,storage-provisioner

```

```bash

export PLATFORM=kratix-labs

kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.12.0/cert-manager.yaml

kubectl apply --filename https://github.com/syntasso/kratix/releases/latest/download/install-all-in-one.yaml

kubectl apply --filename https://github.com/syntasso/kratix/releases/latest/download/config-all-in-one.yaml

kubectl get pods --field-selector=status.phase=Pending --all-namespaces

kubectl get namespace kratix-worker-system
```

### Test Changes
```bash
cd internal/configure-pipeline
cargo build
cargo run pipeline
```

### Build local pipeline image
```bash
eval $(minikube -p $CLUSTER_NAME docker-env)

./internal/scripts/pipeline-image build


```


### Setup (Promise)
```bash
kubectl apply --context $PLATFORM --filename promise.yaml

```
```bash
kubectl --context $WORKER get pods --watch
```

### Setup (Request)
Once the flink operator is running as seen in the previous step you are ready to fulfil a [resource-request](resource-request.yaml) as a Flink job:
```bash
kubectl apply --context $PLATFORM --filename resource-request.yaml
```


### Kratix Verification
```bash 
kubectl --context $PLATFORM get crds flinkdeps.example.promise.syntasso.io

kubectl logs -l=kratix-promise-id=flinkdep -n kratix-platform-system -c flinkdep-promise-pipeline

```

### Teardown (Request)
```bash
kubectl delete --context $PLATFORM --filename resource-request.yaml
```

### Teardown (Promise)
```bash
kubectl delete --context $PLATFORM --filename promise.yaml

```
