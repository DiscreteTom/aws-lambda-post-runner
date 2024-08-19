use lambda_extension::{service_fn, Error, LambdaEvent, NextEvent};
use tokio::process::Command;

async fn my_extension(event: LambdaEvent) -> Result<(), Error> {
  match event.next {
    NextEvent::Shutdown(_e) => {
      Command::new("/bin/bash")
        .arg("-c")
        .arg(format!(
          "kill -SIGUSR1 `cat /tmp/MOCK_LOG_PROCESSOR_PID` && /opt/mock-log-processor-checker"
        ))
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
    }
    NextEvent::Invoke(_e) => {}
  }
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let func = service_fn(my_extension);
  lambda_extension::run(func).await
}
