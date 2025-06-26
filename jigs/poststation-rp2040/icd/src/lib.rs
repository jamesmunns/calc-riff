#![cfg_attr(not(feature = "use-std"), no_std)]

use postcard_rpc::{endpoints, topics, TopicDirection};
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct SleepMillis {
    pub millis: u16,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct SleptMillis {
    pub millis: u16,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub enum LedState {
    Off,
    On,
}

// READ

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct ReadCommand {
    pub addr: u8,
    pub len: u32,
}

#[cfg(not(feature = "use-std"))]
#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct ReadData<'a> {
    pub data: &'a [u8],
}

#[cfg(feature = "use-std")]
#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct ReadData {
    pub data: Vec<u8>,
}

#[cfg(not(feature = "use-std"))]
pub type ReadResult<'a> = Result<ReadData<'a>, I2cError>;

#[cfg(feature = "use-std")]
pub type ReadResult = Result<ReadData, I2cError>;

// WRITE

#[cfg(not(feature = "use-std"))]
#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct WriteCommand<'a> {
    pub addr: u8,
    pub data: &'a [u8],
}

#[cfg(feature = "use-std")]
#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct WriteCommand {
    pub addr: u8,
    pub data: Vec<u8>,
}

pub type WriteResult = Result<(), I2cError>;

// WRITE THEN READ

#[cfg(not(feature = "use-std"))]
#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct WriteReadCommand<'a> {
    pub addr: u8,
    pub tx_data: &'a [u8],
    pub rx_len: u32,
}

#[cfg(feature = "use-std")]
#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct WriteReadCommand {
    pub addr: u8,
    pub tx_data: Vec<u8>,
    pub rx_len: u32,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct I2cError;

// ---

// Endpoints spoken by our device
//
// GetUniqueIdEndpoint is mandatory, the others are examples
endpoints! {
    list = ENDPOINT_LIST;
    | EndpointTy                | RequestTy             | ResponseTy            | Path                          | Cfg                           |
    | ----------                | ---------             | ----------            | ----                          | ---                           |
    | GetUniqueIdEndpoint       | ()                    | u64                   | "poststation/unique_id/get"   |                               |
    | RebootToPicoBoot          | ()                    | ()                    | "i2c-passthru/picoboot/reset" |                               |
    | SleepEndpoint             | SleepMillis           | SleptMillis           | "i2c-passthru/sleep"          |                               |
    | SetLedEndpoint            | LedState              | ()                    | "i2c-passthru/led/set"        |                               |
    | GetLedEndpoint            | ()                    | LedState              | "i2c-passthru/led/get"        |                               |
    | I2cReadEndpoint           | ReadCommand           | ReadResult<'a>        | "jig/sb/i2c/read"             | cfg(not(feature = "use-std")) |
    | I2cReadEndpoint           | ReadCommand           | ReadResult            | "jig/sb/i2c/read"             | cfg(feature = "use-std")      |
    | I2cWriteEndpoint          | WriteCommand<'a>      | WriteResult           | "jig/sb/i2c/write"            | cfg(not(feature = "use-std")) |
    | I2cWriteEndpoint          | WriteCommand          | WriteResult           | "jig/sb/i2c/write"            | cfg(feature = "use-std")      |
    | I2cWriteReadEndpoint      | WriteReadCommand<'a>  | ReadResult<'b>        | "jig/sb/i2c/write-read"       | cfg(not(feature = "use-std")) |
    | I2cWriteReadEndpoint      | WriteReadCommand      | ReadResult            | "jig/sb/i2c/write-read"       | cfg(feature = "use-std")      |
}

// incoming topics handled by our device
topics! {
    list = TOPICS_IN_LIST;
    direction = TopicDirection::ToServer;
    | TopicTy                   | MessageTy     | Path              |
    | -------                   | ---------     | ----              |
}

// outgoing topics handled by our device
topics! {
    list = TOPICS_OUT_LIST;
    direction = TopicDirection::ToClient;
    | TopicTy                   | MessageTy     | Path              | Cfg                           |
    | -------                   | ---------     | ----              | ---                           |
}
