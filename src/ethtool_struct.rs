use std::mem::zeroed;

use crate::ethtool_trait::EthtoolCommand;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_channels {
    pub cmd: u32,
    pub max_rx: u32,
    pub max_tx: u32,
    pub max_other: u32,
    pub max_combined: u32,
    pub rx_count: u32,
    pub tx_count: u32,
    pub other_count: u32,
    pub combined_count: u32,
}

impl std::fmt::Display for ethtool_channels {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Default for ethtool_channels {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

impl EthtoolCommand for ethtool_channels {
    #[inline(always)]
    fn command(&self) -> u32 {
        self.cmd
    }
}
