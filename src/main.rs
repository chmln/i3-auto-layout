#![feature(async_closure)]
use anyhow::{anyhow as err, Result};
use futures::channel::mpsc;
use futures::stream::StreamExt;
use futures::SinkExt;
use tokio_i3ipc::{
    event::{Event, Subscribe, WindowChange},
    msg::Msg,
    reply::Rect,
    I3,
};

#[derive(Debug)]
enum Split {
    Horizontally,
    Vertically,
}

impl From<Rect> for Split {
    fn from(r: Rect) -> Self {
        if r.width > r.height {
            return Split::Horizontally;
        }
        return Split::Vertically;
    }
}

impl Split {
    fn to_string(&self) -> &'static str {
        match self {
            Split::Horizontally => "split h",
            Split::Vertically => "split v",
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (mut send, mut recv) = mpsc::channel::<Split>(0);

    let s_handle = std::thread::spawn(async move || -> Result<()> {
        let mut i3 = I3::connect().await?;
        i3.subscribe([Subscribe::Window]).await?;
        let mut listener = i3.listen();

        while let Some(Ok(Event::Window(ev))) = listener.next().await {
            if let WindowChange::Focus = ev.change {
                let rect = ev.container.window_rect;
                send.send(Split::from(rect)).await?;
            }
        }
        Ok(())
    });

    let r_handle = std::thread::spawn(async move || -> Result<()> {
        let mut i3 = I3::connect().await?;
        while let Some(split) = recv.next().await {
            i3.send_msg_body(Msg::RunCommand, split.to_string())
                .await?;
        }
        Ok(())
    });

    futures::future::try_join(
        s_handle.join().map_err(|_| err!("failed to join thread"))?,
        r_handle.join().map_err(|_| err!("failed to join thread"))?
    ).await?;

    Ok(())
}
