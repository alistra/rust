xfail-stage1
xfail-stage2
xfail-stage3

use std;
import std::sio;

fn main() {
  let cx: sio::ctx = sio::new();
  let srv: sio::server = sio::create_server(cx, "127.0.0.1", 9090);
  sio::close_server(srv);
  sio::destroy(cx);
}