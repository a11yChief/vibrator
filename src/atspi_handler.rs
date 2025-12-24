use atspi_common::events::Event;
use atspi_common::events::object::ObjectEvents;
use atspi_common::events::window::WindowEvents;
use atspi_common::state::State;
use atspi_connection::AccessibilityConnection;
use atspi_proxies::accessible::AccessibleProxy;
use futures_util::stream::StreamExt;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::speech::SpeechEngine;

pub struct AtspiHandler {
    connection: AccessibilityConnection,
    speech: Arc<Mutex<SpeechEngine>>,
}

impl AtspiHandler {
    pub async fn new(speech: Arc<Mutex<SpeechEngine>>) -> Result<Self> {
        let connection = AccessibilityConnection::open().await?;
        
        // Register for events
        connection.register_event::<ObjectEvents>().await?;
        connection.register_event::<WindowEvents>().await?;
        
        Ok(Self { connection, speech })
    }



    pub async fn process_events(&self) -> Result<()> {
        let stream = self.connection.event_stream();
        tokio::pin!(stream);

        while let Some(event) = stream.next().await {
            match event {
                Ok(ev) => {
                    println!("Event: {:?}", ev);
                    if let Err(e) = self.handle_event(&ev).await {
                        eprintln!("Error handling event: {}", e);
                    }
                }
                Err(e) => eprintln!("Error receiving event: {}", e),
            }
        }
        Ok(())
    }

    async fn handle_event(&self, event: &Event) -> Result<()> {
        if let Some(item) = Self::extract_focus_target(event) {
            let proxy = AccessibleProxy::builder(self.connection.connection())
                .destination(item.name.clone())?
                .path(item.path.clone())?
                .build()
                .await?;
            
            let name = proxy.name().await?;
            let role = proxy.get_role().await?;
            
            let text = format!("{} {:?}", name, role);
            
            {
                let mut speech = self.speech.lock().unwrap();
                speech.speak(&text)?;
            }
        }
        Ok(())
    }

    fn extract_focus_target(event: &Event) -> Option<&atspi_common::accessible::Accessible> {
        if let Event::Object(ObjectEvents::StateChanged(ev)) = event {
            if ev.state == State::Focused && ev.enabled == 1 {
                return Some(&ev.item);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atspi_common::events::object::StateChangedEvent;
    use atspi_common::accessible::Accessible;

    #[test]
    fn test_extract_focus_target() {
        let item = Accessible {
            name: ":1.23".into(),
            path: "/org/a11y/atspi/accessible/123".try_into().unwrap(),
        };
        
        let event = Event::Object(ObjectEvents::StateChanged(StateChangedEvent {
            item: item.clone(),
            state: State::Focused,
            enabled: 1,
        }));

        let target = AtspiHandler::extract_focus_target(&event);
        assert!(target.is_some());
        assert_eq!(target.unwrap().name, ":1.23");
    }

    #[test]
    fn test_ignore_non_focus_event() {
        let item = Accessible {
            name: ":1.23".into(),
            path: "/org/a11y/atspi/accessible/123".try_into().unwrap(),
        };
        
        let event = Event::Object(ObjectEvents::StateChanged(StateChangedEvent {
            item: item.clone(),
            state: State::Focused,
            enabled: 0, // Focus lost
        }));

        let target = AtspiHandler::extract_focus_target(&event);
        assert!(target.is_none());
    }
}






