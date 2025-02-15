use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().class_spec::<Numeric>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Numeric", None, None);
    interp.0.borrow_mut().def_class::<Numeric>(spec);
    interp.eval(&include_bytes!("numeric.rb")[..])?;
    trace!("Patched Numeric onto interpreter");
    Ok(())
}

pub struct Numeric;
