use hidapi::{HidDevice, HidApi};
use iced_futures::futures;
use iced_native::subscription::{self, Subscription};

use futures::channel::mpsc;

use futures::stream::StreamExt;
use async_std::future;

use std::fmt;

async fn scan_devices(api: &mut HidApi) -> Option<HidDevice> {
    api.refresh_devices().unwrap();
    for device in api.device_list() {
        if device.vendor_id() == 4617 && device.product_id() == 1 && device.usage_page() == 0xff00 && device.usage() == 1 {
            let d = device.open_device(&api).unwrap();
            return Some(d);
        }
    }

    None
}

async fn is_connected(api: &mut HidApi) -> bool {
    api.refresh_devices().unwrap();
    for device in api.device_list() {
        if device.vendor_id() == 4617 && device.product_id() == 1 && device.usage_page() == 0xff00 && device.usage() == 1 {
            return true;
        }
    }

    false
}

pub fn connect() -> Subscription<Event> {
    struct Connect;

    subscription::unfold(
        std::any::TypeId::of::<Connect>(),
        State::Uninitialized,
        |state| async move {
            match state {
                State::Uninitialized => {
                    let api = hidapi::HidApi::new().unwrap();
                    (Some(Event::Disconnected), State::Disconnected(api))
                }
                State::Disconnected(mut api) => {
                    if let Some(d) = scan_devices(&mut api).await {
                        let (sender, receiver) = mpsc::channel(100);
                            (
                                Some(Event::Connected(Connection(sender))),
                                State::Connected(api, d, receiver),
                            )
                    } else {
                        tokio::time::sleep(
                            tokio::time::Duration::from_secs(1),
                        )
                        .await;

                        (Some(Event::Disconnected), State::Disconnected(api))
                    }
                }
                State::Connected(mut api, device, mut input) => {
                    let command = future::timeout(std::time::Duration::from_secs(1), input.select_next_some()).await;
                    if let Ok(command) = command {
                        match command {
                            Message::Data(data) => {
                                let mut data = data;
                                data[0] = 0;
                                if device.write(&data).is_err() {
                                    (None, State::Disconnected(api))
                                } else {
                                    (None, State::Sent(api, device, input, data))
                                }
                            }

                            _ => (None, State::Connected(api, device, input)),
                        }
                    } else {
                        if is_connected(&mut api).await {
                            (None, State::Connected(api, device, input))
                        } else {
                            (None, State::Disconnected(api))
                        }
                    }
                }
                State::Sent(api, device, input, data) => {
                    let mut buf = [0u8; 65];
                    if device.read_timeout(&mut buf, 1000).is_err() {
                        (None, State::Disconnected(api))
                    } else {
                        (Some(Event::MessageReceived(Message::Data(buf))), State::Connected(api, device, input))
                    }
                }
            }
        },
    )
}

#[allow(clippy::large_enum_variant)]
enum State {
    Uninitialized,
    Disconnected(hidapi::HidApi),
    Connected(
        hidapi::HidApi,
        hidapi::HidDevice,
        mpsc::Receiver<Message>,
    ),
    Sent(
        hidapi::HidApi,
        hidapi::HidDevice,
        mpsc::Receiver<Message>,
        [u8; 65]
    ),
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Uninitialized => write!(f, " Uninitialized"),
            State::Disconnected(_) => write!(f, "Disconnected"),
            State::Connected(_, _, _) => write!(f, "Connected"),
            State::Sent(_, _, _, _) => write!(f, "Sent"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
    Disconnected,
    MessageReceived(Message),
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to echo server");
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected,
    Disconnected,
    Data([u8; 65]),
}

impl Message {
    pub fn new(data: [u8; 65]) -> Option<Self> {
        Some(Self::Data(data))
    }

    pub fn connected() -> Self {
        Message::Connected
    }

    pub fn disconnected() -> Self {
        Message::Disconnected
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Connected => write!(f, "Connected successfully!"),
            Message::Disconnected => {
                write!(f, "Connection lost... Retrying...")
            }
            Message::Data(data) => write!(f, "{:?}", data),
        }
    }
}