# Test

## Deploy

```bash
# run in the root folder of this project
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl
mkdir -p test/layer
cp target/x86_64-unknown-linux-musl/release/aws-lambda-post-runner test/layer
cp scripts/entry.sh test/layer

cd test
sam build
sam deploy # maybe add '-g' for the first time
cd ..
```

In one line:

```bash
RUSTFLAGS="-C link-arg=-s" cargo build --release --target x86_64-unknown-linux-musl && mkdir -p test/layer && cp target/x86_64-unknown-linux-musl/release/aws-lambda-post-runner test/layer && cp scripts/entry.sh test/layer && cd test && sam build && sam deploy && cd ..
```