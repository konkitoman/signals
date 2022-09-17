use std::{
    any::Any,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

pub struct AdvancedSignal<A: 'static, R: 'static> {
    pub methods: Vec<(
        &'static (dyn Fn<(A, Arc<Mutex<Vec<Box<dyn Any + Sync + Send>>>>), Output = R>
                      + Sync
                      + Send),
        Arc<Mutex<Vec<Box<dyn Any + Sync + Send>>>>,
    )>,
}

impl<A, R> Default for AdvancedSignal<A, R> {
    fn default() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
}

impl<A, R> AdvancedSignal<A, R> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn call(&self, arg: A) -> Vec<R>
    where
        A: Clone,
    {
        let mut out = Vec::new();
        let len = self.methods.len();
        for i in 0..len {
            let i = (len - 1) - i;
            let method = &self.methods[i];
            out.push(method.0(arg.clone(), method.1.clone()));
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
            let method = &self.methods[i];
            let arg = arg.clone();
            let method_store = method.1.clone();
            let method = method.0.clone();
            out.push(std::thread::spawn(move || method(arg, method_store)));
        }
        out
    }

    /// Call last function added if is emply will return default value
    pub fn call_last(&self, arg: A) -> R
    where
        R: Default,
    {
        if let Some(method) = self.methods.get(self.methods.len() - 1) {
            return method.0.call((arg, method.1.clone()));
        }
        return Default::default();
    }

    pub fn connect(
        &mut self,
        method: &'static (dyn Fn<(A, Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>), Output = R>
                      + Send
                      + Sync),
        aditionals: Vec<Box<dyn Any + Sync + Send>>,
    ) {
        self.methods
            .push((method, Arc::new(Mutex::new(aditionals))));
    }

    pub fn disconnect(
        &mut self,
        method: &'static (dyn Fn<(A, Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>), Output = R>
                      + Send
                      + Sync),
    ) {
        self.methods
            .retain(|m| m.0 as *const _ != method as *const _)
    }

    pub fn is_empty(&self) -> bool {
        self.methods.len() == 0
    }
}

#[macro_export]
macro_rules! advanced_method {
    (fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = unsafe {std::ptr::read(a.get_row_ptr(_i).unwrap() as *const $var_type)};_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut::<$seccond_type>().unwrap() }; _i+=1;)*
            $body
        }
    };

    (async fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        async fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = unsafe {std::ptr::read(a.get_row_ptr(_i).unwrap() as *const $var_type)};_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut::<$seccond_type>().unwrap() }; _i+=1;)*
            $body
        }
    };

    (pub fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        pub fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = unsafe {std::ptr::read(a.get_row_ptr(_i).unwrap() as *const $var_type)};_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut::<$seccond_type>().unwrap() }; _i+=1;)*
            $body
        }
    };

    (pub async fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        pub async fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any + Send + Sync>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = unsafe {std::ptr::read(a.get_row_ptr(_i).unwrap() as *const $var_type)};_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut::<$seccond_type>().unwrap() }; _i+=1;)*
            $body
        }
    };



    (fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) $body:block) => {
        advanced_method!(fn $name ($($var_name:$var_type),*|$($seccond_name:$seccond_type),*) -> () $body);
    };

    (async fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) $body:block) => {
        advanced_method!(async fn $name ($($var_name:$var_type),*|$($seccond_name:$seccond_type),*) -> () $body);
    };

    (pub fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) $body:block) => {
        advanced_method!(pub fn $name ($($var_name:$var_type),*|$($seccond_name:$seccond_type),*) -> () $body);
    };

    (pub async fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*)  $body:block) => {
        advanced_method!(pub async fn $name ($($var_name:$var_type),*|$($seccond_name:$seccond_type),*) -> () $body);
    };
}
