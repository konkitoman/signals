use std::{
    any::Any,
    sync::{Arc, Mutex},
};

pub struct AdvancedSignal<'a, A, R> {
    pub methods: Vec<(
        &'a dyn Fn<(A, Arc<Mutex<Vec<Box<dyn Any>>>>), Output = R>,
        Arc<Mutex<Vec<Box<dyn Any>>>>,
    )>,
}

impl<'a, A, R> Default for AdvancedSignal<'a, A, R> {
    fn default() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
}

impl<'a, A, R> AdvancedSignal<'a, A, R> {
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
            method.0.call((arg.clone(), method.1.clone()));
        }
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
        method: &'a dyn Fn<(A, Arc<Mutex<Vec<Box<dyn Any>>>>), Output = R>,
        aditionals: Vec<Box<dyn Any>>,
    ) {
        self.methods
            .push((method, Arc::new(Mutex::new(aditionals))));
    }

    pub fn disconnect(
        &mut self,
        method: &'a dyn Fn<(A, Arc<Mutex<Vec<Box<dyn Any>>>>), Output = R>,
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
        fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = *a.get(_i).unwrap();_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut_unchecked::<$seccond_type>() }; _i+=1;)*
            $body
        }
    };

    (async fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        async fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = *a.get(_i).unwrap();_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut_unchecked::<$seccond_type>() }; _i+=1;)*
            $body
        }
    };

    (pub fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        pub fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = *a.get(_i).unwrap();_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut_unchecked::<$seccond_type>() }; _i+=1;)*
            $body
        }
    };

    (pub async fn $name:ident ($($var_name:ident: $var_type:ty),*| $($seccond_name:ident:$seccond_type:ty),*) -> $return:ty $body:block) => {
        pub async fn $name(a: ($($var_type),*), b: Arc<Mutex<Vec<Box<dyn Any>>>>) -> $return {
            let mut _i = 0;
            $(let $var_name: $var_type = *a.get(_i).unwrap();_i+=1;)*
            let mut _i = 0;
            let mut _data = b.lock().unwrap();
            $(let $seccond_name: &mut $seccond_type = unsafe {_data[_i].downcast_mut_unchecked::<$seccond_type>() }; _i+=1;)*
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

mod test {
    use tuple::TupleElements;

    use super::*;

    #[test]
    fn test() {
        let ttm = &32i32 as &dyn Any;
        let t = unsafe { ttm.downcast_ref_unchecked::<u32>() };
        println!("{}", t);
        let mut sig = AdvancedSignal::new();
        let sql = 53i32;
        let mut data = Vec::new();
        let mut data2 = Vec::new();
        data.push(Box::new(sql) as Box<dyn Any>);
        data2.push(Box::new(sql) as Box<dyn Any>);
        sig.connect(&testing, data);
        sig.connect(&testing2, data2);
        sig.call((12, 32));
    }

    advanced_method! {
        pub fn testing (test:i32, dd:i32 | sql: i32) {
            println!("Testing");
        }
    }

    advanced_method! {
        pub fn testing2 (test:i32, dd:i32 | sql: i32) {
            println!("Testing 2");
        }
    }
}
