use crate::{AspectRatio, Resolution};
use crate::platform::{Platform, Window};

pub struct Application {
    platform: Platform,
    window: Window,
    running: bool,
    initilizer: Vec<&'static dyn Fn()>,
    updater: Vec<&'static dyn Fn()>,
    terminator: Vec<&'static dyn Fn()>
}

impl Application{

    pub fn new() -> Self {
        Self {
            platform: Platform::new(),
            window: Window::new(),
            running: false,
            initilizer: Vec::new(),
            updater: Vec::new(),
            terminator: Vec::new()
        }
    }

    /// Function to be executed after the engine initialization.
    /// <br><br> The functions will be called in the registration order
    pub fn initializer(&mut self, funtion: &'static dyn Fn()) -> &mut Self{
        self.initilizer.push(funtion);
        self
    }

    pub fn updater(&mut self, funtion: &'static dyn Fn()) -> &mut Self{
        self.updater.push(funtion);
        self
    }

    /// Function to be executed before the engine termination.
    /// <br><br> The functions will be called in the registration order
    pub fn terminator(&mut self, funtion: &'static dyn Fn()) -> &mut Self{
        self.terminator.push(funtion);
        self
    }

    fn initialize(&mut self) {
        self.platform.initialize();
        self.window.initialize(&self.platform);
        self.window.create(&self.platform, "rust-game-engine", Resolution::FHD, AspectRatio::A16x9);
        self.window.centralize();

        crate::logger::trace("Client Initialize");
        for funtion in &self.initilizer { funtion(); }
    }

    /// The actual engine execution
    pub fn run(&mut self) {
        self.initialize();
        self.window.show();

        self.running = true;
        while self.running {
            self.platform.update();
            for funtion in &self.updater { funtion(); }
        }

        self.window.hide();
        self.terminate();
    }

    fn terminate(&mut self) {
        crate::logger::trace("Client Terminate");
        for funtion in &self.terminator { funtion(); }

        self.window.terminate();
        self.platform.terminate();
    }
}