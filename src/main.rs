extern crate curl;

use curl::http;

fn main (){
  let resp = http::handle()
     .get("https://www.google.com")
     .exec().unwrap();
  let body = String::from_utf8_lossy(resp.get_body());
  let mut part = &body[0..];
  let mut acc = 0;

  loop {
     match part.find("<div") {
         Some(idx) => {
           part = &part[idx+4..];
	   acc += 1;
         },
         None => break
     }
  }
  println!("Number of div's is {}.",acc);
}
