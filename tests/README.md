# Test

## Deploy

```bash
# run in the root folder of this project
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl
mkdir -p tests/layer
cp target/x86_64-unknown-linux-musl/release/aws-lambda-post-runner tests/layer
cp scripts/entry.sh tests/layer

cd tests
sam build
sam deploy # maybe add '-g' for the first time
cd ..
```

In one line:

```bash
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl && mkdir -p tests/layer && cp target/x86_64-unknown-linux-musl/release/aws-lambda-post-runner tests/layer && cp scripts/entry.sh tests/layer && cd tests && sam build && sam deploy && cd ..
```
