use crate::ethtool_trait::EthtoolCommand;
use crate::ifreq;

/// Get number of channels.
pub(crate) const ETHTOOL_GCHANNELS: u32 = 0x0000003C;
/// Set number of channels.
pub(crate) const ETHTOOL_SCHANNELS: u32 = 0x0000003D;

#[cfg(target_env = "musl")]
pub(crate) const SIOCETHTOOL: libc::c_int = 0x00008946;
#[cfg(not(target_env = "musl"))]
pub(crate) const SIOCETHTOOL: libc::c_ulong = 0x00008946;

pub struct EthoolInfo {
    interface_fd: i32,
    interface_name: String,
}

impl EthoolInfo {
    pub fn from_name(interface_name: String) -> Result<Self, ifreq::IfError> {
        let interface_fd = EthoolInfo::new_socket(libc::AF_INET, libc::SOCK_DGRAM, 0, false)?;

        Ok(EthoolInfo {
            interface_fd,
            interface_name,
        })
    }

    pub fn new_socket(
        domain: libc::c_int,
        type_: libc::c_int,
        protocol: libc::c_int,
        non_blocking: bool,
    ) -> Result<i32, ifreq::IfError> {
        let flags = if non_blocking {
            type_ | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK
        } else {
            type_ | libc::SOCK_CLOEXEC
        };

        let result = unsafe { libc::socket(domain, flags, protocol) };
        if result >= 0 {
            Ok(result)
        } else {
            Err(ifreq::IfError::ExecError(std::io::Error::last_os_error()))
        }
    }

    pub fn ioctl<C: EthtoolCommand>(
        &mut self,
        mut ethtool_command: C,
        // callback: impl FnOnce(C) -> Result<C, ifreq::IfError>,
    ) -> Result<C, ifreq::IfError> {
        let mut input_ifreq = ifreq::Ifreq::from_name(&self.interface_name).unwrap();

        input_ifreq.ifr_ifru.ifru_data = &mut ethtool_command as *mut _ as *mut libc::c_char;

        let ret = unsafe { libc::ioctl(self.interface_fd, SIOCETHTOOL, &input_ifreq) };

        if ret == 0 {
            // callback(ethtool_command)
            Ok(ethtool_command)
        } else {
            Err(ifreq::IfError::ExecError(std::io::Error::last_os_error()))
        }
    }
}
