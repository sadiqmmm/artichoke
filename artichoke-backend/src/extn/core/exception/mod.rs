//! # Ruby Exception Hierarchy
//!
//! The built-in subclasses of
//! [`Exception`](https://ruby-doc.org/core-2.6.3/Exception.html) are:
//!
//! - `NoMemoryError`
//! - `ScriptError`
//!   - `LoadError`
//!   - `NotImplementedError`
//!   - `SyntaxError`
//! - `SecurityError`
//! - `SignalException`
//!   - `Interrupt`
//! - `StandardError` -- default for `rescue`
//!   - `ArgumentError`
//!     - `UncaughtThrowError`
//!   - `EncodingError`
//!   - `FiberError`
//!   - `IOError`
//!     - `EOFError`
//!   - `IndexError`
//!     - `KeyError`
//!     - `StopIteration`
//!   - `LocalJumpError`
//!   - `NameError`
//!     - `NoMethodError`
//!   - `RangeError`
//!     - `FloatDomainError`
//!   - `RegexpError`
//!   - `RuntimeError` -- default for `raise`
//!     - `FrozenError`
//!   - `SystemCallError`
//!     - `Errno::*`
//!   - `ThreadError`
//!   - `TypeError`
//!   - `ZeroDivisionError`
//! - `SystemExit`
//! - `SystemStackError`
//! - `fatal` -- impossible to rescue

use artichoke_core::eval::Eval;
#[cfg(feature = "artichoke-debug")]
use backtrace::Backtrace;
use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::class;
use crate::convert::Convert;
use crate::sys;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let borrow = interp.0.borrow();

    let exception_spec = class::Spec::new("Exception", None, None);
    class::Builder::for_spec(interp, &exception_spec)
        .with_super_class(None)
        .define()?;

    let nomemory_spec = class::Spec::new("NoMemoryError", None, None);
    class::Builder::for_spec(interp, &nomemory_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let script_spec = class::Spec::new("ScriptError", None, None);
    class::Builder::for_spec(interp, &script_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let load_spec = class::Spec::new("LoadError", None, None);
    class::Builder::for_spec(interp, &load_spec)
        .with_super_class(Some(&script_spec))
        .define()?;

    let notimplemented_spec = class::Spec::new("NotImplementedError", None, None);
    class::Builder::for_spec(interp, &notimplemented_spec)
        .with_super_class(Some(&script_spec))
        .define()?;

    let syntax_spec = class::Spec::new("SyntaxError", None, None);
    class::Builder::for_spec(interp, &syntax_spec)
        .with_super_class(Some(&script_spec))
        .define()?;

    let security_spec = class::Spec::new("SecurityError", None, None);
    class::Builder::for_spec(interp, &security_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let signal_spec = class::Spec::new("SignalException", None, None);
    class::Builder::for_spec(interp, &signal_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let interrupt_spec = class::Spec::new("Interrupt", None, None);
    class::Builder::for_spec(interp, &interrupt_spec)
        .with_super_class(Some(&signal_spec))
        .define()?;

    // Default for `rescue`.
    let standard_spec = class::Spec::new("StandardError", None, None);
    class::Builder::for_spec(interp, &standard_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let argument_spec = class::Spec::new("ArgumentError", None, None);
    class::Builder::for_spec(interp, &argument_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let uncaughthrow_spec = class::Spec::new("UncaughtThrowError", None, None);
    class::Builder::for_spec(interp, &uncaughthrow_spec)
        .with_super_class(Some(&argument_spec))
        .define()?;

    let encoding_spec = class::Spec::new("EncodingError", None, None);
    class::Builder::for_spec(interp, &encoding_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let fiber_spec = class::Spec::new("FiberError", None, None);
    class::Builder::for_spec(interp, &fiber_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let io_spec = class::Spec::new("IOError", None, None);
    class::Builder::for_spec(interp, &io_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let eof_spec = class::Spec::new("EOFError", None, None);
    class::Builder::for_spec(interp, &eof_spec)
        .with_super_class(Some(&io_spec))
        .define()?;

    let index_spec = class::Spec::new("IndexError", None, None);
    class::Builder::for_spec(interp, &index_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let key_spec = class::Spec::new("KeyError", None, None);
    class::Builder::for_spec(interp, &key_spec)
        .with_super_class(Some(&index_spec))
        .define()?;

    let stopiteration_spec = class::Spec::new("StopIteration", None, None);
    class::Builder::for_spec(interp, &stopiteration_spec)
        .with_super_class(Some(&index_spec))
        .define()?;

    let localjump_spec = class::Spec::new("LocalJumpError", None, None);
    class::Builder::for_spec(interp, &localjump_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let name_spec = class::Spec::new("NameError", None, None);
    class::Builder::for_spec(interp, &name_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let nomethod_spec = class::Spec::new("NoMethodError", None, None);
    class::Builder::for_spec(interp, &nomethod_spec)
        .with_super_class(Some(&name_spec))
        .define()?;

    let range_spec = class::Spec::new("RangeError", None, None);
    class::Builder::for_spec(interp, &range_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let floatdomain_spec = class::Spec::new("FloatDomainError", None, None);
    class::Builder::for_spec(interp, &floatdomain_spec)
        .with_super_class(Some(&range_spec))
        .define()?;

    let regexp_spec = class::Spec::new("RegexpError", None, None);
    class::Builder::for_spec(interp, &regexp_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    // Default `Exception` type for `raise`.
    let runtime_spec = class::Spec::new("RuntimeError", None, None);
    class::Builder::for_spec(interp, &runtime_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let frozen_spec = class::Spec::new("FrozenError", None, None);
    class::Builder::for_spec(interp, &frozen_spec)
        .with_super_class(Some(&runtime_spec))
        .define()?;

    let systemcall_spec = class::Spec::new("SystemCallError", None, None);
    class::Builder::for_spec(interp, &systemcall_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let thread_spec = class::Spec::new("ThreadError", None, None);
    class::Builder::for_spec(interp, &thread_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let type_spec = class::Spec::new("TypeError", None, None);
    class::Builder::for_spec(interp, &type_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let zerodivision_spec = class::Spec::new("ZeroDivisionError", None, None);
    class::Builder::for_spec(interp, &zerodivision_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let systemexit_spec = class::Spec::new("SystemExit", None, None);
    class::Builder::for_spec(interp, &systemexit_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let systemstack_spec = class::Spec::new("SystemStackError", None, None);
    class::Builder::for_spec(interp, &systemstack_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let fatal_spec = class::Spec::new("fatal", None, None);
    class::Builder::for_spec(interp, &fatal_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    drop(borrow);
    let mut borrow = interp.0.borrow_mut();
    borrow.def_class::<Exception>(exception_spec);
    borrow.def_class::<NoMemoryError>(nomemory_spec);
    borrow.def_class::<ScriptError>(script_spec);
    borrow.def_class::<LoadError>(load_spec);
    borrow.def_class::<NotImplementedError>(notimplemented_spec);
    borrow.def_class::<SyntaxError>(syntax_spec);
    borrow.def_class::<SecurityError>(security_spec);
    borrow.def_class::<SignalException>(signal_spec);
    borrow.def_class::<Interrupt>(interrupt_spec);
    borrow.def_class::<StandardError>(standard_spec);
    borrow.def_class::<ArgumentError>(argument_spec);
    borrow.def_class::<UncaughtThrowError>(uncaughthrow_spec);
    borrow.def_class::<EncodingError>(encoding_spec);
    borrow.def_class::<FiberError>(fiber_spec);
    borrow.def_class::<IOError>(io_spec);
    borrow.def_class::<EOFError>(eof_spec);
    borrow.def_class::<IndexError>(index_spec);
    borrow.def_class::<KeyError>(key_spec);
    borrow.def_class::<StopIteration>(stopiteration_spec);
    borrow.def_class::<LocalJumpError>(localjump_spec);
    borrow.def_class::<NameError>(name_spec);
    borrow.def_class::<NoMethodError>(nomethod_spec);
    borrow.def_class::<RangeError>(range_spec);
    borrow.def_class::<FloatDomainError>(floatdomain_spec);
    borrow.def_class::<RegexpError>(regexp_spec);
    borrow.def_class::<RuntimeError>(runtime_spec);
    borrow.def_class::<FrozenError>(frozen_spec);
    borrow.def_class::<SystemCallError>(systemcall_spec);
    borrow.def_class::<ThreadError>(thread_spec);
    borrow.def_class::<TypeError>(type_spec);
    borrow.def_class::<ZeroDivisionError>(zerodivision_spec);
    borrow.def_class::<SystemExit>(systemexit_spec);
    borrow.def_class::<SystemStackError>(systemstack_spec);
    borrow.def_class::<Fatal>(fatal_spec);
    drop(borrow);

    interp.eval(&include_bytes!("exception.rb")[..])?;
    trace!("Patched Exception onto interpreter");
    trace!("Patched core exception hierarchy onto interpreter");
    Ok(())
}

/// Raise implementation for `RubyException` boxed trait objects.
///
/// # Safety
///
/// This function unwinds the stack with `longjmp`, which will ignore all Rust
/// landing pads for panics and exit routines for cleaning up borrows. Callers
/// should ensure that only [`Copy`] items are alive in the current stack frame.
///
/// Because this precondition must hold for all frames between the caller and
/// the closest [`sys::mrb_protect`] landing pad, this function should only be
/// called in the entrypoint into Rust from mruby.
pub unsafe fn raise(interp: Artichoke, exception: impl RubyException) -> ! {
    // Ensure the borrow is out of scope by the time we eval code since
    // Rust-backed files and types may need to mutably borrow the `Artichoke` to
    // get access to the underlying `ArtichokeState`.
    let mrb = interp.0.borrow().mrb;

    let eclass = if let Some(rclass) = exception.rclass() {
        rclass
    } else {
        error!("unable to raise {}", exception.name());
        panic!("unable to raise {}", exception.name());
    };
    let formatargs = interp.convert(exception.message()).inner();
    // `mrb_sys_raise` will call longjmp which will unwind the stack.
    // Any non-`Copy` objects that we haven't cleaned up at this point will
    // leak, so drop everything.
    drop(interp);
    drop(exception);

    sys::mrb_raisef(mrb, eclass, b"%S\0".as_ptr() as *const i8, formatargs);
    unreachable!("mrb_raisef will unwind the stack with longjmp");
}

#[allow(clippy::module_name_repetitions)]
pub trait RubyException
where
    Self: 'static,
{
    fn message(&self) -> &[u8];
    fn name(&self) -> String;
    fn rclass(&self) -> Option<*mut sys::RClass>;
}

macro_rules! ruby_exception_impl {
    ($exception:ident) => {
        pub struct $exception {
            interp: Artichoke,
            message: Cow<'static, [u8]>,
            #[cfg(feature = "artichoke-debug")]
            backtrace: Backtrace,
        }

        impl $exception {
            pub fn new<S>(interp: &Artichoke, message: S) -> Self
            where
                S: Into<Cow<'static, str>>,
            {
                let message = match message.into() {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes()),
                };
                Self {
                    interp: interp.clone(),
                    message,
                    #[cfg(feature = "artichoke-debug")]
                    backtrace: Backtrace::new(),
                }
            }

            pub fn new_raw<S>(interp: &Artichoke, message: S) -> Self
            where
                S: Into<Cow<'static, [u8]>>,
            {
                Self {
                    interp: interp.clone(),
                    message: message.into(),
                    #[cfg(feature = "artichoke-debug")]
                    backtrace: Backtrace::new(),
                }
            }
        }

        #[allow(clippy::use_self)]
        impl From<$exception> for Box<dyn RubyException>
        where
            $exception: RubyException,
        {
            fn from(exception: $exception) -> Box<dyn RubyException> {
                Box::new(exception)
            }
        }

        #[allow(clippy::use_self)]
        impl From<Box<$exception>> for Box<dyn RubyException>
        where
            $exception: RubyException,
        {
            fn from(exception: Box<$exception>) -> Box<dyn RubyException> {
                exception
            }
        }

        impl RubyException for $exception {
            fn message(&self) -> &[u8] {
                self.message.as_ref()
            }

            fn name(&self) -> String {
                self.interp
                    .0
                    .borrow()
                    .class_spec::<Self>()
                    .map(|spec| spec.name().to_owned())
                    .unwrap_or_default()
            }

            fn rclass(&self) -> Option<*mut sys::RClass> {
                self.interp
                    .0
                    .borrow()
                    .class_spec::<Self>()
                    .and_then(|spec| spec.rclass(&self.interp))
            }
        }

        impl fmt::Debug for $exception
        where
            $exception: RubyException,
        {
            #[cfg(feature = "artichoke-debug")]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let classname = self.name();
                let message = String::from_utf8_lossy(self.message());
                write!(f, "{} ({})", classname, message)?;
                write!(f, "\n{:?}", self.backtrace)
            }

            #[cfg(not(feature = "artichoke-debug"))]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let classname = self.name();
                let message = String::from_utf8_lossy(self.message());
                write!(f, "{} ({})", classname, message)
            }
        }

        impl fmt::Display for $exception
        where
            $exception: RubyException,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let classname = self.name();
                let message = String::from_utf8_lossy(self.message());
                write!(f, "{} ({})", classname, message)
            }
        }

        impl error::Error for $exception {
            fn description(&self) -> &str {
                concat!("Ruby Exception: ", stringify!($exception))
            }

            fn cause(&self) -> Option<&dyn error::Error> {
                None
            }
        }
    };
}

impl RubyException for Box<dyn RubyException> {
    fn message(&self) -> &[u8] {
        self.as_ref().message()
    }

    fn name(&self) -> String {
        self.as_ref().name()
    }

    fn rclass(&self) -> Option<*mut sys::RClass> {
        self.as_ref().rclass()
    }
}

impl fmt::Debug for Box<dyn RubyException> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

impl fmt::Display for Box<dyn RubyException> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let classname = self.name();
        let message = String::from_utf8_lossy(self.message());
        write!(f, "{} ({})", classname, message)
    }
}

impl error::Error for Box<dyn RubyException> {
    fn description(&self) -> &str {
        "RubyException"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

ruby_exception_impl!(Exception);
ruby_exception_impl!(NoMemoryError);
ruby_exception_impl!(ScriptError);
ruby_exception_impl!(LoadError);
ruby_exception_impl!(NotImplementedError);
ruby_exception_impl!(SyntaxError);
ruby_exception_impl!(SecurityError);
ruby_exception_impl!(SignalException);
ruby_exception_impl!(Interrupt);
// Default for `rescue`.
ruby_exception_impl!(StandardError);
ruby_exception_impl!(ArgumentError);
ruby_exception_impl!(UncaughtThrowError);
ruby_exception_impl!(EncodingError);
ruby_exception_impl!(FiberError);
ruby_exception_impl!(IOError);
ruby_exception_impl!(EOFError);
ruby_exception_impl!(IndexError);
ruby_exception_impl!(KeyError);
ruby_exception_impl!(StopIteration);
ruby_exception_impl!(LocalJumpError);
ruby_exception_impl!(NameError);
ruby_exception_impl!(NoMethodError);
ruby_exception_impl!(RangeError);
ruby_exception_impl!(FloatDomainError);
ruby_exception_impl!(RegexpError);
// Default `Exception` type for `raise`.
ruby_exception_impl!(RuntimeError);
ruby_exception_impl!(FrozenError);
ruby_exception_impl!(SystemCallError);
// ruby_exception_impl!(Errno::*);
ruby_exception_impl!(ThreadError);
ruby_exception_impl!(TypeError);
ruby_exception_impl!(ZeroDivisionError);
ruby_exception_impl!(SystemExit);
ruby_exception_impl!(SystemStackError);
// Fatal interpreter error. Impossible to rescue.
ruby_exception_impl!(Fatal);

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::file::File;

    use crate::class;
    use crate::exception::Exception;
    use crate::extn::core::exception::RuntimeError;
    use crate::sys;
    use crate::{Artichoke, ArtichokeError};

    struct Run;

    impl Run {
        unsafe extern "C" fn run(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            let interp = unwrap_interpreter!(mrb);
            let exc = RuntimeError::new(&interp, "something went wrong");
            super::raise(interp, exc)
        }
    }

    impl File for Run {
        type Artichoke = Artichoke;

        fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
            let spec = class::Spec::new("Run", None, None);
            class::Builder::for_spec(interp, &spec)
                .add_self_method("run", Self::run, sys::mrb_args_none())
                .define()?;
            interp.0.borrow_mut().def_class::<Self>(spec);
            Ok(())
        }
    }

    #[test]
    fn raise() {
        let interp = crate::interpreter().expect("init");
        Run::require(&interp).unwrap();
        let value = interp.eval(b"Run.run").map(|_| ());
        let expected = Exception::new(
            "RuntimeError",
            "something went wrong",
            Some(vec!["(eval):1".to_owned()]),
            "(eval):1: something went wrong (RuntimeError)",
        );
        assert_eq!(value, Err(ArtichokeError::Exec(expected.to_string())));
    }
}
