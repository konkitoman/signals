use std::thread::JoinHandle;

pub struct Signal<A: 'static, R: 'static> {
    pub methods: Vec<&'static (dyn Fn<A, Output = R> + Sync + Send)>,
}

impl<A, R> Default for Signal<A, R> {
    fn default() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
}

impl<A, R> Signal<A, R> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Calling from front to back, so the last added method will be the first!
    /// This will call all the methods sync!
    pub fn call(&self, arg: A) -> Vec<R>
    where
        A: Clone,
    {
        let mut out = Vec::new();
        let len = self.methods.len();
        for i in 0..len {
            let i = (len - 1) - i;
            out.push(self.methods[i].call(arg.clone()));
        }
        out
    }

    pub fn call_async(&self, arg: A) -> Vec<JoinHandle<R>>
    where
        A: Clone + Sync + Send,
        R: Sync + Send,
    {
        let mut out = Vec::new();
        let len = self.methods.len();
        for i in 0..len {
            let i = (len - 1) - i;
            let arg = arg.clone();
            let func = self.methods[i].clone();

            out.push(std::thread::spawn(move || func.call(arg)));
        }
        out
    }

    /// Call last function added if is emply will return default value
    pub fn call_last(&self, arg: A) -> R
    where
        R: Default,
    {
        if let Some(method) = self.methods.get(self.methods.len() - 1) {
            return method.call(arg);
        }
        return Default::default();
    }

    /// Connect a method
    pub fn connect(&mut self, method: &'static (dyn Fn<A, Output = R> + Send + Sync)) {
        self.methods.push(method);
    }

    /// Remove a method
    pub fn disconnect(&mut self, method: &'static (dyn Fn<A, Output = R> + Send + Sync)) {
        self.methods
            .retain(|m| *m as *const _ != method as *const _)
    }

    pub fn is_empty(&self) -> bool {
        self.methods.len() == 0
    }
}
