use std::ffi::CString;
use std::mem::MaybeUninit;

use windows_sys::core::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::LibraryLoader::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::System::SystemInformation::*;

use crate::{AspectRatio, Resolution, Color};
use crate::platform::{Monitor, Platform, Window};
use crate::time::DateTime;

impl Platform {
    pub fn new() -> Self {
        Self{
            handle: unsafe{ GetModuleHandleA(std::ptr::null()) },
            monitor: Monitor {
                width: unsafe{GetSystemMetrics(SM_CXSCREEN).into()},
                height: unsafe{GetSystemMetrics(SM_CYSCREEN).into()}
            }
        }
    }

    pub fn initialize(&mut self) {
        crate::logger::trace("Platform::initialize()");
    }

    pub fn terminate(&mut self) {
        crate::logger::trace("Platform::terminate()");
    }

    pub fn update(&mut self) {
        unsafe {
            let mut message = std::mem::zeroed();
            while GetMessageA(&mut message, 0, 0, 0) != 0 {
                DispatchMessageA(&message);
            }
        }
    }
}


impl Window {
    const WINDOW_CLASS_NAME: *const u8 = s!("window");

    pub fn new() -> Self {
        Self{
            handle:  0,
            title: String::from("My Window"),
            resolution: Resolution::HD,
            ratio: AspectRatio::A4x3,
            width: 0,
            height: 0,
        }
    }

    pub fn initialize(&mut self, platform: &Platform) {
        crate::logger::trace("Window::initialize()");

        unsafe {
            let wc = WNDCLASSA {
                hCursor: LoadCursorW(0, IDC_ARROW), // Manage the cursor manually
                hInstance: platform.handle,
                style: CS_DBLCLKS, // Get double-clicks
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: LoadIconA(self.handle, IDI_APPLICATION as *const u8),
                hbrBackground: CreateSolidBrush(Color::packed(195, 0, 195, 0)),
                lpszMenuName: std::ptr::null(),
                lpszClassName: Self::WINDOW_CLASS_NAME,
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);
        }
    }

    pub fn create(&mut self, platform: &Platform, title: &str, resolution: Resolution, ratio: AspectRatio) {
        crate::logger::trace("Window::create()");
        self.title = String::from(title);
        self.resolution = resolution;
        self.ratio = ratio;


        let window_ex_style = WS_EX_APPWINDOW;
        let mut window_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
        window_style |= WS_MAXIMIZEBOX;
        window_style |= WS_MINIMIZEBOX;
        window_style |= WS_THICKFRAME;

        self.height = match self.resolution {
            Resolution::HD => 720,
            Resolution::FHD => 1024,
            Resolution::QHD => 1440,
            Resolution::UHD => 2160
        };

        self.width = match self.ratio {
            AspectRatio::A4x3  => (self.height * 4) / 3,
            AspectRatio::A16x9 => (self.height * 16) / 9,
            AspectRatio::A21x9 => (self.height * 21) / 9
        };

        unsafe {
            // Obtain the size of the border.
            let mut border_rect = RECT {left: 0, top: 0, right: 0, bottom: 0};
            AdjustWindowRectEx(&mut border_rect, window_style, 0, window_ex_style);

            // Grow by the size of the OS border.
            self.width += border_rect.right - border_rect.left;
            self.height += border_rect.bottom - border_rect.top;

            self.handle = CreateWindowExA(
                window_ex_style,
                Self::WINDOW_CLASS_NAME,
                CString::new(&*self.title).unwrap().as_ptr() as *const u8,
                window_style,
                0, 0,
                self.width,
                self.height,
                0,
                0,
                platform.handle,
                std::ptr::null(),
            );
            debug_assert!(self.handle != 0);

        }
    }

    pub fn centralize(&mut self) {
        unsafe {
            let witdh: i32 = GetSystemMetrics(SM_CXSCREEN).into();
            let height: i32 = GetSystemMetrics(SM_CYSCREEN).into();
            SetWindowPos(
                self.handle, HWND_TOP,
                (witdh - self.width) / 2,   // X
                (height - self.height) / 2,   // Y
                0, 0, SWP_NOSIZE
            );
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
            UpdateWindow(self.handle);
        }
    }

    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
    }

    pub fn terminate(&mut self) {
        crate::logger::trace("Window::terminate()");
        unsafe {
            DestroyWindow(self.handle);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                crate::event::fire(0);
                println!("WM_DESTROY");
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

impl DateTime {
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