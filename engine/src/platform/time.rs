use crate::time::DateTime;
use std::mem::MaybeUninit;

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::SYSTEMTIME;
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::SystemInformation::GetLocalTime;

impl DateTime {

    #[cfg(target_os = "windows")]
    pub fn now() -> Self {
        let mut now = MaybeUninit::<SYSTEMTIME>::uninit();
        unsafe { GetLocalTime(now.as_mut_ptr()) }
        let st = unsafe { now.assume_init() };

        Self {
            year: st.wYear,
            month: st.wMonth as u8,
            day: st.wDay as u8,

            hour: st.wHour as u8,
            minute: st.wMinute as u8,
            second: st.wSecond as u8,
            millis: st.wMilliseconds
        }
    }
}