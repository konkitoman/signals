#[macro_export]
macro_rules! signal {
    ($a:ty => $b:ty) => {
        Signal<'a, $a, $b>
    };
}

pub struct Signal<'a, A, R> {
    pub methods: Vec<&'a dyn Fn<A, Output = R>>,
}

impl<'a, A, R> Default for Signal<'a, A, R> {
    fn default() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
}

impl<'a, A, R> Signal<'a, A, R> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Call with out return because is multiples results
    pub fn call(&self, arg: A)
    where
        A: Clone,
    {
        let mut tmp_methods = self.methods.clone();
        tmp_methods.reverse();
        for method in tmp_methods.iter() {
            method.call(arg.clone());
        }
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

    pub fn connect(&mut self, method: &'a dyn Fn<A, Output = R>) {
        self.methods.push(method);
    }

    pub fn disconnect(&mut self, method: &'a dyn Fn<A, Output = R>) {
        self.methods
            .retain(|m| *m as *const _ != method as *const _)
    }

    pub fn is_empty(&self) -> bool {
        self.methods.len() == 0
    }
}
