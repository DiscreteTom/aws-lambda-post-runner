use aws_lambda_runtime_proxy::{LambdaRuntimeApiClient, Proxy};
use tokio::process::Command;

#[tokio::main]
async fn main() {
  let cmd = std::env::var("AWS_LAMBDA_POST_RUNNER_COMMAND")
    .expect("No command found for aws-lambda-post-runner");

  Proxy::default()
    .spawn()
    .await
    .server
    .serve(move |req| {
      let cmd = cmd.clone();

      async move {
        let path = req.uri().path();
        let is_handler_response =
          path.starts_with("/2018-06-01/runtime/invocation/") && path.ends_with("/response");

        let res = LambdaRuntimeApiClient::forward(req).await;

        if is_handler_response {
          // current request is a handler's response,
          // before proceed, run the command
          Command::new("/bin/bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .await
            .unwrap();
        }

        res
      }
    })
    .await
}
