extern crate mio;
pub fn main() {
    use mio::{Events, Poll};
    use std::time::Duration;

    let mut events = Events::with_capacity(1024);
    let poll = Poll::new()?;

    assert_eq!(0, events.len());

    // Register `Evented` handles with `poll`

    poll.poll(&mut events, Some(Duration::from_millis(100)))?;

    for event in &events {
        println!("event={:?}", event);
    }
}
