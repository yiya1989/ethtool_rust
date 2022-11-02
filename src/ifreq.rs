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

impl std::error::Error for IfError {
    #[inline(always)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

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

#[repr(C)]
#[derive(Copy, Clone)]
pub union IfreqIfrn {
    /// `ifr_name`.
    pub ifrn_name: [libc::c_char; libc::IFNAMSIZ],
}

impl Default for IfreqIfrn {
    #[inline(always)]
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IfreqIfru {
    /// `ifr_data`.
    ///
    /// Data for use by network interface.
    pub(crate) ifru_data: *mut libc::c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct Ifreq {
    pub ifr_name: [libc::c_char; libc::IFNAMSIZ],
    pub ifr_ifru: IfreqIfru,
}

impl Ifreq {
    fn new() -> Self {
        Ifreq {
            ..Default::default()
        }
    }

    pub fn from_name(name: &str) -> Result<Ifreq, IfError> {
        let mut ifreq_wrapper = Ifreq::new();
        ifreq_wrapper.set_name(name)?;
        Ok(ifreq_wrapper)
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), IfError> {
        if name.len() >= libc::IF_NAMESIZE {
            Err(IfError::IfNameToLong)
        } else {
            for (i, c) in name.as_bytes().iter().enumerate() {
                self.ifr_name[i] = *c as libc::c_char;
            }
            Ok(())
        }
    }
}

impl Default for Ifreq {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
