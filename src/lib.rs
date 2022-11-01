// use ifstructs::ifreq;
use libc::{
    self, __c_anonymous_ifr_ifru, c_char, IFNAMSIZ, SIOCETHTOOL, SOCK_CLOEXEC, SOCK_NONBLOCK,
};
use libc::{c_int, socket};
use std::error;
use std::mem::zeroed;

mod ifreq;
use ifreq::IfreqWrapper;

mod command_trait;

use command_trait::EthtoolCommand;

pub const ETHTOOL_SFEATURES: u32 = 59;
pub const ETHTOOL_GCHANNELS: u32 = 60;

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

pub fn get_channel_max_combined<E: std::error::Error + 'static>(
    interface_name: &str,
) -> Result<i32, ifreq::IfError<E>> {
    let max_combined = 0;

    let mut ethtool_config = ethtool_channels {
        cmd: ETHTOOL_GCHANNELS,
        ..Default::default()
    };

    // ifr_ifrn: ifreq_ifrn {
    //     ifrn_name: network_interface_name.into(),
    // },
    // ifr_ifru: ifreq_ifru {
    //     ifru_data: &mut command as *mut C as *mut c_void,

    let pointer = interface_name.as_ptr();
    let ifr_name = unsafe { *(pointer as *const [c_char; IFNAMSIZ]) };

    // IfReq::

    // let ifr = ifreq {
    //     ifr_name: ifr_name,
    //     ifr_ifru: __c_anonymous_ifr_ifru {
    //         ifru_data: &mut ethtool_config as *mut _ as *mut c_char,
    //     },
    // };

    let ifreqWrapperR = IfreqWrapper::from_name(interface_name);
    let ifreqWrapper = match ifreqWrapperR {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    ifreqWrapper.set_ifru_data(ethtool_config)?;

    if interface_name == "eth0" {
        let loop_device_fd = new_socket(libc::AF_INET, libc::SOCK_DGRAM, 0, false).unwrap();

        let ret = unsafe { libc::ioctl(loop_device_fd, SIOCETHTOOL, &(ifreqWrapper.to_ifreq())) };
        if ret == 0 {
            Ok(ethtool_config.max_combined as i32)
        } else {
            Err(ifreq::IfError::ExecError(std::io::Error::last_os_error()))
        }
    } else {
        Ok(max_combined)
    }
}

pub fn new_socket<E: std::error::Error + 'static>(
    domain: c_int,
    type_: c_int,
    protocol: c_int,
    non_blocking: bool,
) -> Result<i32, ifreq::IfError<E>> {
    let flags = if non_blocking {
        type_ | SOCK_CLOEXEC | SOCK_NONBLOCK
    } else {
        type_ | SOCK_CLOEXEC
    };

    let result = unsafe { socket(domain, flags, protocol) };
    if result >= 0 {
        Ok(result)
    } else {
        Err(ifreq::IfError::ExecError(std::io::Error::last_os_error()))
    }
}
