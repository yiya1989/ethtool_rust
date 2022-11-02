use libc::ifreq;
use std::mem::zeroed;

#[derive(Debug)]
pub enum IfError {
    IfNameToLong,
    ExecError(std::io::Error),
    Other(String),
}

impl std::fmt::Display for IfError {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

// impl<E: error::Error + 'static> error::Error for IfError {
//     #[inline(always)]
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         use self::IfError::*;

//         match self {
//             &ExecError(ref error) => Some(error),

//             &IfNameToLong => None,

//             &Other(..) => None,
//         }
//     }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn error::Error> {
//         self.source()
//     }
// }

// impl IfError {
//     #[inline(always)]
//     pub(crate) fn map_error<E2: error::Error + 'static>(self) -> IfError {
//         use self::IfError::*;

//         match self {
//             ExecError(e) => unsafe { hint::unreachable_unchecked() },

//             IfNameToLong => IfNameToLong,

//             Other(other) => Other(other),
//         }
//     }
// }
pub struct IfreqWrapper {
    pub ifreq: ifreq,
}

impl IfreqWrapper {
    fn new() -> Self {
        IfreqWrapper {
            ..Default::default()
        }
    }

    // pub fn from_name<E: error::Error + 'static>(name: &str) -> Result<ifreq_wrapper, IfError<E>> {
    //     let ifreq_wrapper = ifreq_wrapper::new();
    //     let r = ifreq_wrapper.set_name(name)?;
    //     Ok(ifreq_wrapper)
    // }
    pub fn from_name(name: &str) -> Result<IfreqWrapper, IfError> {
        let mut ifreq_wrapper = IfreqWrapper::new();
        ifreq_wrapper.set_name(name)?;
        Ok(ifreq_wrapper)
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), IfError> {
        if name.len() >= libc::IF_NAMESIZE {
            Err(IfError::IfNameToLong)
        } else {
            for (i, c) in name.as_bytes().iter().enumerate() {
                self.ifreq.ifr_name[i] = *c as libc::c_char;
            }
            Ok(())
        }
    }
}

impl Default for IfreqWrapper {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
