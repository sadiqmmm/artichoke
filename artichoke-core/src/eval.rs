//! Run code on an Artichoke interpreter.

use crate::value::Value;
use crate::ArtichokeError;

/// Marker trait for a context used by [`Eval`].
pub trait Context {}

/// Interpreters that implement [`Eval`] expose methods for injecting code and
/// extracting [`Value`]s from the interpereter.
///
/// Implementations are expected to maintain a stack of `Context` objects
/// that maintain filename context across nested invocations of
/// [`Eval::eval`].
pub trait Eval {
    /// Concrete type for eval context.
    type Context: Context;

    /// Concrete type for return values from eval.
    type Value: Value;

    /// Filename of the top eval context.
    const TOP_FILENAME: &'static [u8] = b"(eval)";

    /// Eval code on the artichoke interpreter using the current `Context`.
    fn eval(&self, code: &[u8]) -> Result<Self::Value, ArtichokeError>;

    /// Eval code on the artichoke interpreter using the current `Context`.
    ///
    /// Exceptions will unwind past this call.
    fn unchecked_eval(&self, code: &[u8]) -> Self::Value;

    /// Peek at the top of the [`Context`] stack.
    fn peek_context(&self) -> Option<Self::Context>;

    /// Push an `Context` onto the stack.
    fn push_context(&self, context: Self::Context);

    /// Pop an `Context` from the stack.
    fn pop_context(&self);
}
