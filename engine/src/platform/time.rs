use std::mem::MaybeUninit;
use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::System::SystemInformation::GetLocalTime;

#[derive(Copy, Clone, Debug)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millis: u16
}

pub fn now() -> DateTime { DateTime::now() }
impl DateTime {

    #[cfg(target_os = "windows")]
    fn now() -> Self {
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