/// The Trace trait which needs to be implemented on garbage collected objects
pub trait Trace {
    /// Mark all contained Gcs
    unsafe fn trace(&self);
    /// Increment the root-count of all contained Gcs
    unsafe fn root(&self);
    /// Decrement the root-count of all contained Gcs
    unsafe fn unroot(&self);
}

/// This simple rule implements the trace methods such with empty
/// implementations - use this for marking types as not containing any Trace types!
#[macro_export]
macro_rules! empty_trace {
    () => {
        #[inline]
        unsafe fn trace(&self) {}
        #[inline]
        unsafe fn root(&self) {}
        #[inline]
        unsafe fn unroot(&self) {}
    }
}

/// This rule implements the trace method. You define a this parameter name, and
/// pass in a body, the body should call `mark` on every traceable element inside
/// the body, and the mark implementation will automatically delegate to the correct
/// method on the argument.
#[macro_export]
macro_rules! custom_trace {
    ($this:ident, $body:expr) => {
        #[inline]
        unsafe fn trace(&self) {
            #[inline]
            unsafe fn mark<T: Trace>(it: &T) {
                (*it).trace();
            }
            let $this = self;
            $body
        }
        #[inline]
        unsafe fn root(&self) {
            #[inline]
            unsafe fn mark<T: Trace>(it: &T) {
                (*it).root();
            }
            let $this = self;
            $body
        }
        #[inline]
        unsafe fn unroot(&self) {
            #[inline]
            unsafe fn mark<T: Trace>(it: &T) {
                (*it).unroot();
            }
            let $this = self;
            $body
        }
    }
}

impl<T> Trace for &'static T {
    empty_trace!();
}

impl Trace for i8  { empty_trace!(); }
impl Trace for u8  { empty_trace!(); }
impl Trace for i16 { empty_trace!(); }
impl Trace for u16 { empty_trace!(); }
impl Trace for i32 { empty_trace!(); }
impl Trace for u32 { empty_trace!(); }
impl Trace for i64 { empty_trace!(); }
impl Trace for u64 { empty_trace!(); }

impl Trace for f32 { empty_trace!(); }
impl Trace for f64 { empty_trace!(); }

impl Trace for String { empty_trace!(); }

impl<T: Trace> Trace for Box<T> {
    custom_trace!(this, {
        mark(&**this);
    });
}

impl<T: Trace> Trace for Vec<T> {
    custom_trace!(this, {
        for e in this {
            mark(e);
        }
    });
}

impl<T: Trace> Trace for Option<T> {
    custom_trace!(this, {
        if let Some(ref v) = *this {
            mark(v);
        }
    });
}
