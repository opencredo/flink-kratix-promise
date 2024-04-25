# Kratix-based Flink Promise Operator

This project provides a Kubernetes operator for managing Flink jobs using a Kratix-backed state store.

[![Entrypoint](https://github.com/opencredo/promise-flink/actions/workflows/entrypoint.yml/badge.svg)](https://github.com/opencredo/promise-flink/actions/workflows/entrypoint.yml)


## Prerequisites

- A running Kubernetes cluster
- Flink Kubernetes Operator installed (See: [https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/](https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/))
- Docker environment with the ability to build images for both amd64 and arm64 architectures.



### Test Changes
```bash
export WORKSPACE="your path to repo her" # e.g. /Users/username/work/oc
export KRATIX_WORKFLOW_TYPE="promise" or "resource"

cd internal/configure-pipeline
cargo build
cargo test
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
