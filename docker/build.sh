#!/bin/bash

docker buildx build -t redstonewizard/kernel-builder:latest --push --platform linux/amd64,linux/arm64 .
