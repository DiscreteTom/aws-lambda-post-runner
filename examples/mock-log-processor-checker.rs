use posixmq::PosixMq;

// this program just wait for the ack message from the mock-log-processor

fn main() {
  let done = PosixMq::open("/mock-log-processor-done").unwrap();
  let mut buffer = vec![0; done.attributes().unwrap().max_msg_len];
  let msg = done.recv(&mut buffer).unwrap();
  println!("{:?}", msg);
}
