use atspi_common::events::Event;
use atspi_common::events::object::ObjectEvents;
use atspi_common::events::window::WindowEvents;
use atspi_common::events::keyboard::KeyboardEvents;
use atspi_common::state::State;
use atspi_connection::AccessibilityConnection;
use atspi_proxies::accessible::AccessibleProxy;
use futures_util::stream::StreamExt;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::speech::SpeechEngine;
use crate::key_listener::KeyListener;
use zbus::zvariant::ObjectPath;

pub struct AtspiHandler {
    connection: AccessibilityConnection,
    speech: Arc<Mutex<SpeechEngine>>,
}

impl AtspiHandler {
    pub async fn new(speech: Arc<Mutex<SpeechEngine>>) -> Result<Self> {
        println!("Connecting to AT-SPI...");
        let connection = AccessibilityConnection::open().await?;
        println!("Connected. Registering events...");
        
        // Register for events
        connection.register_event::<ObjectEvents>().await?;
        connection.register_event::<WindowEvents>().await?;
        connection.register_event::<KeyboardEvents>().await?;
        println!("Standard events registered.");
        
        // Setup KeyListener
        let insert_pressed = Arc::new(Mutex::new(false));
        let key_listener = KeyListener {
            speech: speech.clone(),
            insert_pressed: insert_pressed.clone(),
        };
        
        let listener_path = ObjectPath::try_from("/org/a11y/atspi/listeners/vibrator")?;
        connection.connection().object_server().at(&listener_path, key_listener).await?;
        println!("KeyListener object served.");
        
        // Register for all keys asynchronously to avoid blocking the event loop
        let connection_clone = connection.connection().clone();
        let listener_path_clone = listener_path.clone();
        
        tokio::spawn(async move {
            println!("Spawning registration task...");
            let raw_registry = match zbus::Proxy::new(
                &connection_clone,
                "org.a11y.atspi.Registry",
                "/org/a11y/atspi/registry/deviceeventcontroller",
                "org.a11y.atspi.DeviceEventController"
            ).await {
                Ok(proxy) => proxy,
                Err(e) => {
                    eprintln!("Failed to create registry proxy: {}", e);
                    return;
                }
            };

            // keys: Empty vector means "All Keys"
            let keys: Vec<(i32, i32, &str, i32)> = vec![];
            
            // mask: 0 means no modifiers (listen to keys without modifiers, or all if handled by registry?)
            // Actually, for global listeners, mask usually filters. 0 might mean "no specific modifier required"?
            // But with empty keys, it listens to everything.
            let mask = 0u32;

            // event_types: Bitmask of events to listen to.
            // 1 << 0 = KeyPressed
            // 1 << 1 = KeyReleased
            // 3 = Both
            let event_types = 3u32; 
            
            // mode: (synchronous, preemptive, global)
            // synchronous=true: We can consume events.
            // preemptive=true: We get events before the application.
            // global=false: We are not using legacy global hooks (Wayland/modern AT-SPI uses device event controller).
            let mode = (true, true, false); 

            println!("Calling RegisterKeystrokeListener...");
            if let Err(e) = raw_registry.call_method(
                "RegisterKeystrokeListener",
                &(&listener_path_clone, &keys, mask, event_types, mode)
            ).await {
                eprintln!("Failed to register keystroke listener: {}", e);
            } else {
                println!("RegisterKeystrokeListener success.");
            }
        });
        
        Ok(Self { connection, speech })
    }

    pub async fn process_events(&mut self) -> Result<()> {
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

    async fn handle_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Object(ObjectEvents::StateChanged(ev)) => {
                if ev.state == State::Focused && ev.enabled == 1 {
                    if let Some(item) = Self::extract_focus_target(event) {
                        let proxy = AccessibleProxy::builder(self.connection.connection())
                            .destination(item.name.clone())?
                            .path(item.path.clone())?
                            .build()
                            .await?;
                        
                        let name = proxy.name().await.unwrap_or_default();
                        let role = proxy.get_role().await.unwrap_or(atspi_common::Role::Unknown);
                        
                        let text = format!("{} {:?}", name, role);
                        
                        {
                            let mut speech = self.speech.lock().unwrap();
                            speech.speak(&text)?;
                        }
                    }
                }
            }
            Event::Keyboard(ev) => {
                println!("Keyboard Event: {:?}", ev);
            }
            _ => {}
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






