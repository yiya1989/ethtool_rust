pub(crate) trait EthtoolCommand {
    fn command(&self) -> u32;
}
