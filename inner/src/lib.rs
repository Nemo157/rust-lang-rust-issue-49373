#![feature(conservative_impl_trait)]

extern crate futures;

use std::time::Duration;
use futures::{future, Future};

pub struct Timer;

impl Timer {
    pub fn timeout<'a>(&'a mut self, _duration: Duration) -> impl Future<Item=(), Error=!> + 'a {
        future::ok(())
    }
}
