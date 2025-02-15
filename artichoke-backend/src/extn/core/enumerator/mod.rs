use artichoke_core::eval::Eval;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.0.borrow().class_spec::<Enumerator>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Enumerator", None, None);
    interp.0.borrow_mut().def_class::<Enumerator>(spec);
    interp.eval(&include_bytes!("enumerator.rb")[..])?;
    interp.eval(&include_bytes!("lazy.rb")[..])?;
    trace!("Patched Enumerator onto interpreter");
    trace!("Patched Enumerator::Lazy onto interpreter");
    Ok(())
}

pub struct Enumerator;
