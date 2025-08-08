use std::task::{Context, Poll};
use std::time::Duration;
use std::{borrow::Cow, pin::Pin};

use indicatif::{ProgressBar, ProgressStyle};
use pin_project::pin_project;

#[pin_project]
pub struct ProgressFuture<F: Future> {
    #[pin]
    future: F,
    progress: ProgressBar,
}

impl<F: Future> Future for ProgressFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        this.future.poll(cx).map(|value| {
            this.progress
                .finish_with_message(format!("{} ✅", this.progress.message()));
            value
        })
    }
}

pub trait Progress: Sized + Future {
    fn progress(self, message: impl Into<Cow<'static, str>>) -> ProgressFuture<Self> {
        let style = ProgressStyle::with_template("{spinner} {wide_msg}").unwrap();
        let progress = ProgressBar::new(20).with_style(style).with_message(message);
        progress.enable_steady_tick(Duration::from_millis(100));

        ProgressFuture {
            future: self,
            progress,
        }
    }
}

impl<F: Future> Progress for F {}
