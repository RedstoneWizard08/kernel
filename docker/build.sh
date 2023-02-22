#!/bin/bash

docker buildx build -t redstonewizard/kernel-builder:latest --push --file Dockerfile .
