use core::task::Poll;
use pasts::{Exec, Loop, Task};

type Exit = ();

struct State {
    tasks: [Task<&'static str>; 2],
}

impl State {
    fn completion(&mut self, (id, val): (usize, &str)) -> Poll<Exit> {
        println!("Received message from {}, completed task: {}", id, val);
        Poll::Ready(())
    }

    fn event_loop(&mut self, exec: Exec<Self, Exit>) -> impl Loop<Exit> {
        exec.poll(&mut self.tasks, Self::completion)
    }
}

async fn run() {
    let mut tasks = State {
        tasks: [Box::pin(async { "Hello" }), Box::pin(async { "World" })],
    };

    pasts::event_loop(&mut tasks, State::event_loop).await;
}

fn main() {
    pasts::block_on(run())
}
