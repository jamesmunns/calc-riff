use std::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use embedded_hal_async::i2c::{Error, ErrorType, I2c, Operation};
use picocalc_jig_icd::*;
use poststation_sdk::{connect, PoststationClient};
use tokio::time::interval;

struct I2cDev {
    serial: u64,
    client: PoststationClient,
    ctr: AtomicU32,
}

#[derive(Debug)]
enum HostI2CError {
    ConnectionError,
    DeviceError,
    NotYetSupported,
}

impl Error for HostI2CError {
    fn kind(&self) -> embedded_hal_async::i2c::ErrorKind {
        embedded_hal_async::i2c::ErrorKind::Other
    }
}

impl ErrorType for I2cDev {
    type Error = HostI2CError;
}

impl I2c for I2cDev {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        // TODO: impl operations for real. For now, it'd probably be easy to just
        // support read/write/write_then_read.
        match operations {
            [] => Ok(()),
            [Operation::Read(buf)] => self.read(address, buf).await,
            [Operation::Write(buf)] => self.write(address, buf).await,
            [Operation::Write(tx), Operation::Read(rx)] => self.write_read(address, tx, rx).await,
            _ => Err(HostI2CError::NotYetSupported),
        }
    }

    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let Ok(res) = self
            .client
            .proxy_endpoint::<I2cReadEndpoint>(
                self.serial,
                self.ctr(),
                &ReadCommand {
                    addr: address,
                    len: read.len() as u32,
                },
            )
            .await
        else {
            return Err(HostI2CError::ConnectionError);
        };

        let Ok(data) = res else {
            return Err(HostI2CError::DeviceError);
        };

        read.copy_from_slice(&data.data);
        Ok(())
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let res = self
            .client
            .proxy_endpoint::<I2cWriteEndpoint>(
                self.serial,
                self.ctr(),
                &WriteCommand {
                    addr: address,
                    data: write.to_vec(),
                },
            )
            .await;

        match res {
            Ok(Ok(())) => Ok(()),
            Ok(Err(I2cError)) => Err(HostI2CError::DeviceError),
            Err(_) => Err(HostI2CError::ConnectionError),
        }
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let res = self
            .client
            .proxy_endpoint::<I2cWriteReadEndpoint>(
                self.serial,
                self.ctr(),
                &WriteReadCommand {
                    addr: address,
                    tx_data: write.to_vec(),
                    rx_len: read.len() as u32,
                },
            )
            .await;

        match res {
            Ok(Ok(resp)) => {
                read.copy_from_slice(&resp.data);
                Ok(())
            }
            Ok(Err(I2cError)) => Err(HostI2CError::DeviceError),
            Err(_) => Err(HostI2CError::ConnectionError),
        }
    }
}

impl I2cDev {
    pub fn new(client: PoststationClient, serial: u64) -> Self {
        Self {
            serial,
            client,
            ctr: AtomicU32::new(0),
        }
    }

    #[inline(always)]
    fn ctr(&self) -> u32 {
        self.ctr.fetch_add(1, Ordering::Relaxed)
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    const SERIAL: u64 = 0xE66430A64B335337u64;
    let client = connect("127.0.0.1:51837").await.unwrap();
    let mut i2c = I2cDev::new(client, SERIAL);

    // Use our client device as if it was a local I2C port with
    // embedded-hal-async traits
    let mut data = [0u8; 2];
    // #define I2C_KBD_ADDR 0x1F
    let addr = 0x1F;

    let mut ticker = interval(Duration::from_millis(50));
    let mut state = KeyState::default();
    loop {
        ticker.tick().await;
        // Something?
        i2c.write(addr, &[0x09]).await.unwrap();
        tokio::time::sleep(Duration::from_millis(16)).await;
        i2c.read(addr, &mut data).await.unwrap();

        let rpt = state.update(data);
        if let Some(rpt) = rpt {
            if matches!(rpt.evt, KeyEvent::Hold(_)) {
                continue;
            }
            println!("{rpt:02X?}");
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Key {
    Char(char),
    LeftDPad,
    UpDPad,
    DownDPad,
    RightDPad,
    Func1,
    Func2,
    Func3,
    Func4,
    Func5,
    Func6,
    Func7,
    Func8,
    Func9,
    Func10,
    Esc,
    Tab,
    CapsLk,
    Del,
    Back,
    Brk,
    Home,
    End,
    Enter,
    Ins,
    Other(u8),
}

#[derive(Debug)]
enum KeyEvent {
    Press(Key),
    Release(Key),
    Hold(Key),
    Other([u8; 2]),
}

#[derive(Debug)]
struct KeyReport {
    ctrl: bool,
    shift: bool,
    alt: bool,
    evt: KeyEvent,
}

#[derive(Default)]
struct KeyState {
    ctrl: bool,
    lshift: bool,
    rshift: bool,
    alt: bool,
}

const SPECIALS: &[(u8, Key)] = &[
    (0xB4, Key::LeftDPad),
    (0xB5, Key::UpDPad),
    (0xB6, Key::DownDPad),
    (0xB7, Key::RightDPad),
    (0x81, Key::Func1),
    (0x82, Key::Func2),
    (0x83, Key::Func3),
    (0x84, Key::Func4),
    (0x85, Key::Func5),
    (0x86, Key::Func6),
    (0x87, Key::Func7),
    (0x88, Key::Func8),
    (0x89, Key::Func9),
    (0x90, Key::Func10),
    (0xB1, Key::Esc),
    (0x09, Key::Tab),
    (0xC1, Key::CapsLk),
    (0xD4, Key::Del),
    (0x08, Key::Back),
    (0xD0, Key::Brk),
    (0xD2, Key::Home),
    (0xD5, Key::End),
    (0x0A, Key::Enter),
    (0xD1, Key::Ins),
];

impl KeyState {
    fn report(&self, evt: KeyEvent) -> KeyReport {
        KeyReport {
            ctrl: self.ctrl,
            shift: self.lshift | self.rshift,
            alt: self.alt,
            evt,
        }
    }

    fn update(&mut self, data: [u8; 2]) -> Option<KeyReport> {
        if data == [0, 0] {
            return None;
        }
        match data {
            [1, 0xA2] => {
                self.lshift = true;
                None
            }
            [3, 0xA2] => {
                self.lshift = false;
                None
            }
            [1, 0xA3] => {
                self.rshift = true;
                None
            }
            [3, 0xA3] => {
                self.rshift = false;
                None
            }
            [1, 0xA5] => {
                self.ctrl = true;
                None
            }
            [3, 0xA5] => {
                self.ctrl = false;
                None
            }
            [1, 0xA1] => {
                self.alt = true;
                None
            }
            [3, 0xA1] => {
                self.alt = false;
                None
            }
            [1, n] if n.is_ascii() && !n.is_ascii_control() => {
                let ch: char = n.into();
                Some(self.report(KeyEvent::Press(Key::Char(ch))))
            }
            [2, n] if n.is_ascii() && !n.is_ascii_control() => {
                let ch: char = n.into();
                Some(self.report(KeyEvent::Hold(Key::Char(ch))))
            }
            [3, n] if n.is_ascii() && !n.is_ascii_control() => {
                let ch: char = n.into();
                Some(self.report(KeyEvent::Release(Key::Char(ch))))
            }
            [a, b] => {
                if let Some(f) =
                    SPECIALS
                        .iter()
                        .find_map(|(idx, k)| if b == *idx { Some(k) } else { None })
                {
                    match a {
                        1 => Some(self.report(KeyEvent::Press(*f))),
                        2 => Some(self.report(KeyEvent::Hold(*f))),
                        3 => Some(self.report(KeyEvent::Release(*f))),
                        _ => Some(self.report(KeyEvent::Other(data))),
                    }
                } else {
                    match a {
                        1 => Some(self.report(KeyEvent::Press(Key::Other(b)))),
                        2 => Some(self.report(KeyEvent::Hold(Key::Other(b)))),
                        3 => Some(self.report(KeyEvent::Release(Key::Other(b)))),
                        _ => Some(self.report(KeyEvent::Other(data))),
                    }
                }
            }
        }
    }
}
