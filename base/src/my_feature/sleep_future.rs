use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;


struct SleepFuture {
    duration: Duration,
    state: Arc<Mutex<State>>,
}

struct State {
    waker: Option<Waker>,
    inner_state: InnerState,
}
#[derive(Eq, PartialEq)]
enum InnerState {
    Init,
    Sleeping,
    Done,
}
impl SleepFuture {
    fn new(duration: Duration) -> Self {
        Self {
            duration,
            state: Arc::new(Mutex::new(State {
                waker: None,
                inner_state: InnerState::Init
            }))
        }
    }
}
impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();
        if guard.inner_state == InnerState::Done {
            return Poll::Ready(());
        }
        if guard.inner_state == InnerState::Init {
            guard.inner_state == InnerState::Sleeping;
            let duration = self.duration;
            let state_clone = Arc::clone(&self.state);
            std::thread::spawn(move || {
                println!("state sleeping.");
                thread::sleep(duration);
                let mut guard = state_clone.lock().unwrap();
                guard.inner_state = InnerState::Done;
                if let Some(waker) = guard.waker.take() {
                    waker.wake_by_ref();
                    println!("waker.wake_by_ref();");
                }
                println!("wake up");
            });
        }
        println!("before cp cx waker");
        guard.waker = Some(cx.waker().clone());
        println!("after cp cx waker");
        Poll::Pending
    }
}
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::my_feature::sleep_future::SleepFuture;

    #[tokio::test] // 使用 tokio 的测试属性
    async fn test_one() {
        let a = 1;
        SleepFuture::new(Duration::from_secs(1)).await;
        println!("{} = 1", a);
    }
}