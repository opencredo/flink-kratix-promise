# Kratix-based Flink Promise Operator

This project provides a Kubernetes operator for managing Flink jobs using a Kratix-backed state store.

[![Docker](https://github.com/opencredo/promise-flink/actions/workflows/docker-publish.yml/badge.svg?branch=main)](https://github.com/opencredo/promise-flink/actions/workflows/docker-publish.yml)

[![Super-Linter](https://github.com/opencredo/promise-flink/actions/workflows/lint.yml/badge.svg)](https://github.com/marketplace/actions/super-linter)

## Prerequisites

- A running Kubernetes cluster
- Flink Kubernetes Operator installed (See: [https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/](https://nightlies.apache.org/flink/flink-kubernetes-operator-docs-main/))
- Docker environment with the ability to build images for both amd64 and arm64 architectures.
