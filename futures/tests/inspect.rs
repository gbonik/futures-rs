#![feature(pin, arbitrary_self_types, futures_api)]

use futures::executor::block_on;
use futures::future;
use futures::prelude::*;

#[test]
fn smoke() {
    let mut counter = 0;

    {
        let work = future::ready::<i32>(40).inspect(|val| { counter += *val; });
        assert_eq!(block_on(work), 40);
    }

    assert_eq!(counter, 40);
}
