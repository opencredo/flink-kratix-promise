# Kratix-based Flink Promise Operator

This project provides a Kubernetes operator for managing Flink jobs using a Kratix-backed state store.

[![Entrypoint](https://github.com/opencredo/promise-flink/actions/workflows/entrypoint.yml/badge.svg)](https://github.com/opencredo/promise-flink/actions/workflows/entrypoint.yml)

[![Super-Linter](https://github.com/opencredo/promise-flink/actions/workflows/lint.yml/badge.svg)](https://github.com/marketplace/actions/super-linter)

## Prerequisites

- A running Kubernetes cluster
- Flink Kubernetes Operator installed (See: [https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/](https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/))
- Docker environment with the ability to build images for both amd64 and arm64 architectures.

### Choose implementation

There are 2 versions of the apache flink promise. 
1. Shell 
2. Rust. 

There is no difference for the user of apache flink promise. Depending on your choice of version please go either in shell or rust folder and then execute below statements.

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
kubectl --context $PLATFORM get crds flinks.example.promise.syntasso.io

kubectl logs -l=kratix-promise-id=flink -n kratix-platform-system -c flink-promise-pipeline

```

### Teardown (Request)
```bash
kubectl delete --context $PLATFORM --filename resource-request.yaml
```

### Teardown (Promise)
```bash
kubectl delete --context $PLATFORM --filename promise.yaml

```