use posixmq::PosixMq;
use std::time::Duration;
use tokio::{
  io::{stdin, AsyncBufReadExt, BufReader},
  signal::unix::{signal, SignalKind},
  time::sleep,
};

// as you can see this example doesn't use the post runner as a dependency
// which means you can write these logics in any other programming language

#[tokio::main]
async fn main() {
  // write the pid to a file so other processes can send signals to this process by pid
  let pid = std::process::id();
  std::fs::write("/tmp/MOCK_LOG_PROCESSOR_PID", pid.to_string()).unwrap();

  // other process will send SIGUSR1 to this process when an invocation is done
  let mut sig_usr1 = signal(SignalKind::user_defined1()).unwrap();
  // other process will send SIGUSR2 to this process when the lambda is initialized
  let mut sig_usr2 = signal(SignalKind::user_defined2()).unwrap();

  // read logs from stdin
  let mut lines = BufReader::new(stdin()).lines();
  // store logs in memory. you can also write logs to a file in `/tmp`
  let mut log_buffer = vec![];

  // create a linux mq to send ack when all logs are processed
  let done_mq = PosixMq::create("/mock-log-processor-done").unwrap();

  let mut initialized = false;

  loop {
    tokio::select! {
      // use biased select to we will always process lines first
      biased;

      line = lines.next_line() => {
        if !initialized {
          // logs during initialization will be printed to stdout
          println!("{}", line.unwrap().unwrap());
        } else {
          // append buffer. you can also filter/transform the log before appending
          // e.g. record a timestamp, transform to JSON, etc.
          log_buffer.push(format!("got: {}", line.unwrap().unwrap()));
        }
      }
      _ = sig_usr1.recv() => {
        let log = log_buffer.join("\n");
        log_buffer.clear();

        // process the log, e.g. you can upload it to s3.
        // you can also do the post processing in post runner's command.
        // here we just sleep for 5 seconds to simulate the processing
        sleep(Duration::from_secs(5)).await;
        println!("{}", log);

        // finally, reply to post runner to stop suppressing the /invocation/next
        done_mq.send(0, b"1").unwrap();
      }
      _ = sig_usr2.recv() => {
        initialized = true;
      }
    }
  }
}
