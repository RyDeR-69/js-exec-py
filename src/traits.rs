use ion::{
    BigInt as JSBigInt, Function as JSFunction, Object as JSObject, OwnedKey, Value as JSValue,
};

/// A trait for extending the lifetime of JavaScript values.
///
/// # Safety
///
/// This trait should ONLY be used in contexts where:
///
/// 1. The JavaScript runtime is guaranteed to outlive all values derived from it.
/// 2. The values are never used after the runtime is dropped.
///
/// The implementation in this crate relies on the fact that the JavaScript runtime
/// is stored in a thread-local and is only dropped when the program exits.
///
/// Using this trait incorrectly can lead to use-after-free and other memory safety issues.
pub trait ExtendLifetime {
    type Output;

    /// Extends the lifetime of a value to 'static.
    ///
    /// # Safety
    ///
    /// This method is safe to use ONLY because the JavaScript runtime context
    /// lives for the entire duration of the program, and JavaScript values
    /// are only valid within their creating context.
    fn extend_lifetime(self) -> Self::Output;
}

impl<'a> ExtendLifetime for OwnedKey<'a> {
    type Output = OwnedKey<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<OwnedKey<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for JSValue<'a> {
    type Output = JSValue<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<JSValue<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for JSObject<'a> {
    type Output = JSObject<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<JSObject<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for JSBigInt<'a> {
    type Output = JSBigInt<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<JSBigInt<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for JSFunction<'a> {
    type Output = JSFunction<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<JSFunction<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for ion::Symbol<'a> {
    type Output = ion::Symbol<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<ion::Symbol<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for ion::PropertyDescriptor<'a> {
    type Output = ion::PropertyDescriptor<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<ion::PropertyDescriptor<'a>, Self::Output>(self) }
    }
}

impl<'a> ExtendLifetime for ion::PropertyKey<'a> {
    type Output = ion::PropertyKey<'static>;
    fn extend_lifetime(self) -> Self::Output {
        unsafe { std::mem::transmute::<ion::PropertyKey<'a>, Self::Output>(self) }
    }
}
