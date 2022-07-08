use std::{
    alloc::System,
    cell::{Cell, RefCell, UnsafeCell},
    io::{BufReader, BufWriter, Bytes, Write},
    rc::{Rc, Weak},
    sync::{
        atomic::{
            AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16,
            AtomicU32, AtomicU64, AtomicU8, AtomicUsize,
        },
        mpsc::{Receiver, Sender, SyncSender},
        Arc, Mutex,
    },
    thread::{JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
    time::{Duration, Instant, SystemTime},
};

pub trait TGetPtr {
    const N: usize;

    fn get_row_ptr(&self, index: usize) -> Option<*const usize>;
}

macro_rules! trupl_intrp {
    ($def:ident) => {
        $def!(
            {T0.0, T1.1}
            {T0.0, T1.1, T2.2}
            {T0.0, T1.1, T2.2, T3.3}
            {T0.0, T1.1, T2.2, T3.3, T4.4}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12, T13.13}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12, T13.13, T14.14}
            {T0.0, T1.1, T2.2, T3.3, T4.4, T5.5, T6.6, T7.7, T8.8, T9.9, T10.10, T11.11, T12.12, T13.13, T14.14, T15.15}
        );
    };
}

macro_rules! impl_truple {
    ($( { $($T:ident.$idx:tt),* } )*) => ($(
        impl<$($T: 'static,)*> TGetPtr for ($($T),*){
            const N: usize = $($idx+)*0;

            fn get_row_ptr(&self, index: usize) -> Option<*const usize> {
                match index{
                    $($idx => Some((&self.$idx as *const _) as *const usize),)*
                    _ => None
                }
            }
        }
    )*)
}

#[macro_export]
macro_rules! impl_get_ptr {
    (<$($TT:ident: $TTP:ident),*> $T:ty) => {
        impl<$($TT:$TTP),*> TGetPtr for $T {
            const N: usize = 1;

            fn get_row_ptr(&self, index: usize) -> Option<*const usize> {
                match index {
                    0 => Some((self as *const _) as *const usize),
                    _ => None,
                }
            }
        }
    };

    (<$($TT:ident),*> $T:ty) => {
        impl<$($TT),*> TGetPtr for $T {
            const N: usize = 1;

            fn get_row_ptr(&self, index: usize) -> Option<*const usize> {
                match index {
                    0 => Some((self as *const _) as *const usize),
                    _ => None,
                }
            }
        }
    };

    ($T:ty) => {
        impl TGetPtr for $T {
            const N: usize = 1;

            fn get_row_ptr(&self, index: usize) -> Option<*const usize> {
                match index {
                    0 => Some((self as *const _) as *const usize),
                    _ => None,
                }
            }
        }
    };

    ($($T:ty),*) => ($(impl_get_ptr!($T);)*);
}

trupl_intrp!(impl_truple);

impl_get_ptr!(i8, i32, i64, i128, isize);
impl_get_ptr!(u8, u32, u64, u128, usize);
impl_get_ptr!(f32, f64);

impl_get_ptr!(bool, AtomicBool);

impl_get_ptr!(AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize);
impl_get_ptr!(AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize);

impl_get_ptr!(Instant, Duration, System, SystemTime, String, &str);
impl_get_ptr!(Thread, ThreadId);
impl_get_ptr!(Scope<'_, '_>);
impl_get_ptr!(<T> JoinHandle<T>);
impl_get_ptr!(<T> ScopedJoinHandle<'_, T>);

impl_get_ptr!(<T> Vec<T>);
impl_get_ptr!(<T> std::vec::Drain<'_, T>);

impl_get_ptr!(<T> Option<T>);
impl_get_ptr!(<T> Mutex<T>);
impl_get_ptr!(<T> Arc<T>);
impl_get_ptr!(<T> Rc<T>);
impl_get_ptr!(<T> Receiver<T>);
impl_get_ptr!(<T> Sender<T>);
impl_get_ptr!(<T> SyncSender<T>);
impl_get_ptr!(<T> Weak<T>);
impl_get_ptr!(<T> Cell<T>);
impl_get_ptr!(<T> RefCell<T>);
impl_get_ptr!(<T> UnsafeCell<T>);
impl_get_ptr!(<T> Bytes<T>);
impl_get_ptr!(<T> BufReader<T>);
impl_get_ptr!(<T: Write> BufWriter<T>);
