use libc;
use std::{error, hint, mem::zeroed};

use crate::command_trait::EthtoolCommand;
use libc::ifreq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfError<E: error::Error + 'static> {
    IfNameToLong,
    ExecError(E),
    Other(String),
}

impl<E: error::Error + 'static> std::fmt::Display for IfError<E> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<E: error::Error + 'static> error::Error for IfError<E> {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use self::IfError::*;

        match self {
            &ExecError(ref error) => Some(error),

            &IfNameToLong => None,

            &Other(..) => None,
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

impl IfError<std::convert::Infallible> {
    #[inline(always)]
    pub(crate) fn map_error<E2: error::Error + 'static>(self) -> IfError<E2> {
        use self::IfError::*;

        match self {
            ExecError(e) => unsafe { hint::unreachable_unchecked() },

            IfNameToLong => IfNameToLong,

            Other(other) => Other(other),
        }
    }
}
pub struct IfreqWrapper {
    ifreq: ifreq,
}

impl IfreqWrapper {
    fn new() -> Self {
        IfreqWrapper {
            ..Default::default()
        }
    }

    // pub fn from_name<E: error::Error + 'static>(name: &str) -> Result<IfreqWrapper, IfError<E>> {
    //     let ifreqWrapper = IfreqWrapper::new();
    //     let r = ifreqWrapper.set_name(name)?;
    //     Ok(ifreqWrapper)
    // }
    pub fn from_name<E: error::Error + 'static>(name: &str) -> Result<IfreqWrapper, IfError<E>> {
        let ifreqWrapper = IfreqWrapper::new();
        let r = ifreqWrapper.set_name(name)?;
        Ok(ifreqWrapper)
    }

    pub fn set_name<E: error::Error + 'static>(&mut self, name: &str) -> Result<(), IfError<E>> {
        if name.len() >= libc::IF_NAMESIZE {
            Err(IfError::IfNameToLong)
        } else {
            for (i, c) in name.as_bytes().iter().enumerate() {
                self.ifreq.ifr_name[i] = *c as libc::c_char;
            }
            Ok(())
        }
    }

    pub fn set_ifru_data<C: EthtoolCommand, E: error::Error + 'static>(
        &mut self,
        command: C,
    ) -> Result<(), IfError<E>> {
        self.ifreq.ifr_ifru.ifru_data = ifru_data;
        Ok(())
    }

    pub fn to_ifreq(&self) -> ifreq {
        self.ifreq
    }
}

impl Default for IfreqWrapper {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
