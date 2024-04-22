# Mock Log Processor

In this example we will demonstrate how to capture logs from a lambda function and process them (e.g. filter/transform the logs or send to a different service like S3).

## How This Works

We will use [`mock-log-processor-entry.sh`](./mock-log-processor-entry.sh) as the runtime wrapper script to enable the post runner and forward logs to [`mock-log-processor`](./mock-log-processor.rs). The `mock-log-processor` will create `/tmp/MOCK_LOG_PROCESSOR_PID` so we can send signals to it in the post runner command.

During lambda invocations, logs are stored in memory by the `mock-log-processor`. You can also change this behaviour as you need (e.g. store logs in a file or send to a different service).

When an invocation is done, the post runner command will be executed, which will send a signal to the `mock-log-processor` to indicate that the processing for current invocation has finished. The `mock-log-processor` will use linux mq to send a `done` message after all logs are processed (here we sleep for 5 seconds to mock the processing). 

The post runner command will use [`mock-log-processor-checker`](./mock-log-processor-checker.rs) to wait for the `done` message. When the `done` is received, the post runner will stop suppressing the `/invocation/next` request.

## Deploy

```bash
cargo build --release && cargo build --examples --release && mkdir -p examples/layer && cp target/release/aws-lambda-post-runner examples/layer && cp target/release/examples/mock-log-processor examples/layer && cp target/release/examples/mock-log-processor-checker examples/layer && chmod +x examples/mock-log-processor-entry.sh && cp examples/mock-log-processor-entry.sh examples/layer && cd examples && sam build -t mock-log-processor.yaml && sam deploy --stack-name MockLogProcessor --resolve-s3 --capabilities CAPABILITY_IAM  && cd ..
```

## Delete

```bash
rm -rf examples/layer && rm -rf examples/.aws-sam && cd examples && sam delete --stack-name MockLogProcessor --no-prompts && cd ..
```
