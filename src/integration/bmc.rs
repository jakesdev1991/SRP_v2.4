pub struct SpdmClient;
impl SpdmClient {
    pub fn new() -> Self { SpdmClient }
    pub fn get_pcp_blob(&self) -> Option<Vec<u8>> {
        Some(vec![0xDE, 0xAD, 0xBE, 0xEF])
    }
}
