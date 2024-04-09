use aws_lambda_runtime_proxy::{LambdaRuntimeApiClient, Proxy};
use tokio::process::Command;

#[tokio::main]
async fn main() {
    Proxy::default()
        .spawn()
        .await
        .server
        .serve(|req| async {
            if req.uri().path() == "/2018-06-01/runtime/invocation/next" {
                let cmd = std::env::var("AWS_LAMBDA_POST_RUNNER_COMMAND")
                    .expect("No command found for aws-lambda-post-runner");
                Command::new("/bin/bash")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .await
                    .unwrap();
            }

            LambdaRuntimeApiClient::forward(req).await
        })
        .await
}
