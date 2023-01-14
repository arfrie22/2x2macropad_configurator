use iced_futures::futures;
use iced_native::subscription::{self, Subscription};

use futures::channel::mpsc;

use async_std::future;
use futures::stream::StreamExt;

use std::fmt;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use sysinfo::DiskExt;
use sysinfo::SystemExt;

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    url: String,
    id: u32,
    node_id: String,
    name: String,
    label: String,
    uploader: Author,
    content_type: String,
    state: String,
    size: u32,
    download_count: u32,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: u32,
    author: Author,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<Asset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
}

async fn scan_devices() -> Option<PathBuf> {
    let sys = sysinfo::System::new_all();
    let mut pico_drive = None;
    for disk in sys.disks() {
        let mount = disk.mount_point();

        if mount.join("INFO_UF2.TXT").is_file() {
            // println!("Found pico uf2 disk {}", &mount.to_string_lossy());
            pico_drive = Some(mount.to_owned());
            break;
        }
    }

    pico_drive
}

pub fn connect() -> Subscription<Event> {
    struct Connect;

    subscription::unfold(
        std::any::TypeId::of::<Connect>(),
        State::NoDeviceFound,
        |state| async move {
            match state {
                State::NoDeviceFound => {
                    if let Some(pico) = scan_devices().await {
                        let (sender, receiver) = mpsc::channel(100);
                        (
                            Some(Event::Connected(Connection(sender))),
                            State::DeviceFound(pico, receiver),
                        )
                    } else {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                        (None, State::NoDeviceFound)
                    }
                }
                State::DeviceFound(pico, mut input) => {
                    let command = future::timeout(
                        std::time::Duration::from_secs(1),
                        input.select_next_some(),
                    )
                    .await;
                    if let Ok(command) = command {
                        match command {
                            Message::UploadToDevice(version) => {
                                let client = reqwest::Client::builder()
                                    .user_agent("2x2macropad_configurator firmware updater")
                                    .build()
                                    .unwrap();

                                let releases = client.get("https://api.github.com/repos/arfrie22/2x2macropad_firmware/releases")
                                    .send()
                                    .await
                                    .unwrap()
                                    .json::<Vec<Release>>()
                                    .await
                                    .unwrap();

                                let firmware = if version.is_some() {
                                    unimplemented!()
                                } else {
                                    client
                                        .get(
                                            releases
                                                .first()
                                                .unwrap()
                                                .assets
                                                .first()
                                                .unwrap()
                                                .browser_download_url
                                                .clone(),
                                        )
                                        .send()
                                        .await
                                        .unwrap()
                                        .bytes()
                                        .await
                                        .unwrap()
                                };

                                let deployed_path = pico.join("out.uf2");
                                let mut f = File::create(&deployed_path).unwrap();

                                f.write_all(&firmware).unwrap();

                                (Some(Event::Disconnected), State::NoDeviceFound)
                            }

                            Message::Close => (None, State::NoDeviceFound),

                            _ => (None, State::DeviceFound(pico, input)),
                        }
                    } else {
                        if pico.exists() {
                            (None, State::DeviceFound(pico, input))
                        } else {
                            (Some(Event::Disconnected), State::NoDeviceFound)
                        }
                    }
                }
            }
        },
    )
}

#[allow(clippy::large_enum_variant)]
enum State {
    NoDeviceFound,
    DeviceFound(PathBuf, mpsc::Receiver<Message>),
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::NoDeviceFound => write!(f, "NoDeviceFound"),
            State::DeviceFound(path, _) => write!(f, "DeviceFound({:?})", path),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
    Disconnected,
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
    UploadToDevice(Option<String>),
    Close,
}

impl Message {
    pub fn upload() -> Option<Self> {
        Some(Message::UploadToDevice(None))
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
            Message::UploadToDevice(version) => {
                if let Some(version) = version {
                    write!(f, "Uploading firmware version {}", version)
                } else {
                    write!(f, "Uploading latest firmware")
                }
            }
            Message::Close => write!(f, "Closing connection"),
        }
    }
}
