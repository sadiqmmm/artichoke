use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

use crate::class;
use crate::eval::Context;
use crate::fs::Filesystem;
use crate::module;
use crate::sys::{self, DescribeState};

// NOTE: ArtichokeState assumes that it it is stored in `mrb_state->ud` wrapped in a
// [`Rc`] with type [`Artichoke`] as created by [`crate::interpreter`].
pub struct State {
    pub mrb: *mut sys::mrb_state,
    pub ctx: *mut sys::mrbc_context,
    classes: HashMap<TypeId, Box<class::Spec>>,
    modules: HashMap<TypeId, Box<module::Spec>>,
    pub vfs: Filesystem,
    pub(crate) context_stack: Vec<Context>,
    pub active_regexp_globals: usize,
    symbol_cache: HashMap<Cow<'static, [u8]>, sys::mrb_sym>,
    captured_output: Option<String>,
    #[cfg(feature = "artichoke-random")]
    prng: crate::extn::core::random::Random,
}

impl State {
    /// Create a new [`State`] from a [`sys::mrb_state`] and
    /// [`sys::mrbc_context`] with an
    /// [in memory virtual filesystem](Filesystem).
    pub fn new(mrb: *mut sys::mrb_state, ctx: *mut sys::mrbc_context, vfs: Filesystem) -> Self {
        Self {
            mrb,
            ctx,
            classes: HashMap::default(),
            modules: HashMap::default(),
            vfs,
            context_stack: vec![],
            active_regexp_globals: 0,
            symbol_cache: HashMap::default(),
            captured_output: None,
            #[cfg(feature = "artichoke-random")]
            prng: crate::extn::core::random::new(None),
        }
    }

    #[cfg(feature = "artichoke-random")]
    pub fn prng(&self) -> &crate::extn::core::random::Random {
        &self.prng
    }

    #[cfg(feature = "artichoke-random")]
    pub fn prng_mut(&mut self) -> &mut crate::extn::core::random::Random {
        &mut self.prng
    }

    pub fn capture_output(&mut self) {
        self.captured_output = Some(String::default());
    }

    pub fn get_and_clear_captured_output(&mut self) -> String {
        self.captured_output
            .replace(String::default())
            .unwrap_or_default()
    }

    pub fn print(&mut self, s: &str) {
        if let Some(ref mut captured_output) = self.captured_output {
            captured_output.push_str(s);
        } else {
            print!("{}", s);
            let _ = io::stdout().flush();
        }
    }

    pub fn puts(&mut self, s: &str) {
        if let Some(ref mut captured_output) = self.captured_output {
            captured_output.push_str(s);
            captured_output.push('\n');
        } else {
            println!("{}", s);
        }
    }

    /// Close a [`State`] and free underlying mruby structs and memory.
    pub fn close(&mut self) {
        unsafe {
            // At this point, the only refs to the smart poitner wrapping the
            // state are stored in the `mrb_state->ud` pointer and any
            // `MRB_TT_DATA` objects in the mruby heap.
            //
            // To clean up:
            //
            // - Save the raw pointer to the `Artichoke` from the user data.
            // - Free the mrb context.
            // - Close the interpreter which frees every object in the heap and
            //   drops the strong count on the Rc to 1.
            // - Rematerialize the `Rc`.
            // - Drop the `Rc` which drops the strong count to 0 and frees the
            //   state.
            // - Set the userdata pointer to null.
            // - Set context and mrb properties to null.
            if self.mrb.is_null() {
                return;
            }
            let ptr = (*self.mrb).ud;
            if ptr.is_null() {
                return;
            }
            // Free mrb data structures
            sys::mrbc_context_free(self.mrb, self.ctx);
            sys::mrb_close(self.mrb);
            // Cleanup dangling pointers
            self.ctx = std::ptr::null_mut();
            self.mrb = std::ptr::null_mut();
        };
    }

    /// Create a class definition bound to a Rust type `T`. Class definitions
    /// have the same lifetime as the [`State`] because the class def owns the
    /// `mrb_data_type` for the type, which must be long-lived. Class defs are
    /// stored by [`TypeId`] of `T`.
    pub fn def_class<T>(&mut self, spec: class::Spec)
    where
        T: Any,
    {
        self.classes.insert(TypeId::of::<T>(), Box::new(spec));
    }

    /// Retrieve a class definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_class`].
    pub fn class_spec<T>(&self) -> Option<&class::Spec>
    where
        T: Any,
    {
        self.classes.get(&TypeId::of::<T>()).map(Box::as_ref)
    }

    /// Create a module definition bound to a Rust type `T`. Module definitions
    /// have the same lifetime as the [`State`]. Module defs are stored by
    /// [`TypeId`] of `T`.
    pub fn def_module<T>(&mut self, spec: module::Spec)
    where
        T: Any,
    {
        self.modules.insert(TypeId::of::<T>(), Box::new(spec));
    }

    /// Retrieve a module definition from the state bound to Rust type `T`.
    ///
    /// This function returns `None` if type `T` has not had a class spec
    /// registered for it using [`State::def_module`].
    pub fn module_spec<T>(&self) -> Option<&module::Spec>
    where
        T: Any,
    {
        self.modules.get(&TypeId::of::<T>()).map(Box::as_ref)
    }

    pub fn sym_intern<T>(&mut self, sym: T) -> sys::mrb_sym
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let mrb = self.mrb;
        let sym = sym.into();
        let ptr = sym.as_ref().as_ptr();
        let len = sym.as_ref().len();
        let interned = self
            .symbol_cache
            .entry(sym)
            .or_insert_with(|| unsafe { sys::mrb_intern_static(mrb, ptr as *const i8, len) });
        *interned
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mrb.debug())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mrb.info())
    }
}
