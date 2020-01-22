use anyhow::{Error, Result};
use futures::{future, StreamExt};
use tokio::sync::mpsc;
use tokio_i3ipc::{
    event::{Event, Subscribe, WindowChange},
    msg::Msg,
    reply::{Node, NodeLayout, Rect},
    I3,
};

#[rustfmt::skip]
fn split_rect(r: Rect) -> &'static str {
    if r.width > r.height { "split h" }
    else { "split v" }
}

// walk the tree and determine if `window_id` has tabbed parent
fn find_child(node: &Node, window_id: usize, tabbed: bool) -> bool {
    if tabbed || node.id == window_id {
        tabbed
    } else {
        node.nodes.iter().any(|child| {
            find_child(
                child,
                window_id,
                match node.layout {
                    NodeLayout::Tabbed | NodeLayout::Stacked => true,
                    _ => false,
                },
            )
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (mut send, mut recv) = mpsc::channel::<&'static str>(1);

    let s_handle = tokio::spawn(async move {
        let mut event_listener = {
            let mut i3 = I3::connect().await?;
            i3.subscribe([Subscribe::Window]).await?;
            i3.listen()
        };

        let i3 = &mut I3::connect().await?;

        while let Some(Ok(Event::Window(window_data))) = event_listener.next().await {
            if WindowChange::Focus == window_data.change {
                let tree = i3.get_tree().await?;
                let is_tabbed = match window_data.container.layout {
                    NodeLayout::Tabbed | NodeLayout::Stacked => true,
                    _ => false,
                };

                if !find_child(&tree, window_data.container.id, is_tabbed) {
                    send.send(split_rect(window_data.container.window_rect))
                        .await?;
                }
            }
        }
        Ok::<_, Error>(())
    });

    let r_handle = tokio::spawn(async move {
        let mut i3 = I3::connect().await?;
        while let Some(cmd) = recv.recv().await {
            i3.send_msg_body(Msg::RunCommand, cmd).await?;
        }
        Ok::<_, Error>(())
    });

    future::try_join_all(vec![s_handle, r_handle]).await?;
    Ok(())
}
