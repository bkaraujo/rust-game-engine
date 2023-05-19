pub struct Application {
    running: bool,
    initilizer: Vec<&'static dyn Fn()>,
    updater: Vec<&'static dyn Fn()>,
    terminator: Vec<&'static dyn Fn()>
}

impl Application {

    pub fn new() -> Self {
        Self {
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

    /// The actual engine execution
    pub fn run(&mut self) {
        for funtion in &self.initilizer { funtion(); }

        // self.running = true;
        while self.running {
            for funtion in &self.updater { funtion(); }
        }

        for funtion in &self.terminator { funtion(); }
    }
}