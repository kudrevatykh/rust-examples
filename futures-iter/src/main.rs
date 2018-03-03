extern crate futures;

use futures::prelude::*;
use futures::future;
use futures::stream;
fn main() {
    let stream = stream::iter_ok::<_, ()>(vec![17, 19]).map(|x| future::ok::<u32, ()>(x+3));
    let st_map = stream.map(|x| x.into_stream()).flatten().collect();
    assert_eq!(Ok(vec![20,22]), st_map.wait());
    println!("success");
}
