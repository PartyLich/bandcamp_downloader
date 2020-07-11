use std::sync::Arc;

use futures::channel::mpsc;
use futures::stream::StreamExt;
use tokio::sync::Mutex;

use crate::ui::Message;

type State = Arc<Mutex<mpsc::Receiver<Message>>>;

/// Convenience function to generate a Subscription using the LogRecipe
pub fn log(receiver: State) -> iced::Subscription<Message> {
    iced::Subscription::from_recipe(LogRecipe::from(receiver))
}

/// Generate subscription friendly futures from a Receiver
async fn log_generator(state: State) -> Option<(Message, State)> {
    let message = state.lock().await.next().await;
    match message {
        Some(message) => Some((message, state)),
        None => {
            // We do not let the stream die, as it would start a new download repeatedly if the
            // user is not careful in case of errors.
            let _: () = iced::futures::future::pending().await;

            None
        }
    }
}

#[derive(Debug)]
pub struct LogRecipe {
    pub receiver: State,
}

impl From<State> for LogRecipe {
    fn from(receiver: State) -> Self {
        Self { receiver }
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for LogRecipe
where
    H: std::hash::Hasher,
{
    type Output = Message;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(self.receiver, log_generator))
    }
}
