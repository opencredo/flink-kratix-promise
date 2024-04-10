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


```bash
./internal/scripts/pipeline-image build
kubectl apply --context $PLATFORM --filename promise.yaml
kubectl logs -l=kratix-promise-id=flinkdep -n kratix-platform-system

kubectl delete --context $PLATFORM --filename promise.yaml