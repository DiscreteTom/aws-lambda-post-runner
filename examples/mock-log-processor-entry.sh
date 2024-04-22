#!/bin/bash

# use `env -u _LAMBDA_TELEMETRY_LOG_FD` to ensure logs are printed to stdout/stderr
# use `|&` to redirect both stdout and stderr to the log processor
env -u _LAMBDA_TELEMETRY_LOG_FD /opt/aws-lambda-post-runner "$@" |& /opt/mock-log-processor