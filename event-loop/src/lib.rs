#[allow(unused_variables)]
pub trait Events {
    fn on_connect(&self) {}
    fn on_error(&self, err: &str) {}
    fn on_read(&self, resp: &[u8]) {}
    fn on_shutdown(&self) {}
    fn on_pre_read(&self) {}
    fn on_post_read(&self) {}
}
