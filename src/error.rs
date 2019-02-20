/// An error raised by the SDK
#[derive(Debug)]
pub enum SDKError {
    Decode(&'static str), // Failed decoding someting
}
