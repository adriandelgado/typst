//! The standard library for the _Typst_ language.

use crate::func::Scope;

mod align;
mod styles;

/// Useful imports for creating your own functions.
pub mod prelude {
    pub use crate::func::{Command, FuncCommands, Function};
    pub use crate::layout::{layout_tree, Layout, LayoutContext, MultiLayout};
    pub use crate::layout::{LayoutError, LayoutResult};
    pub use crate::parsing::{parse, ParseContext, ParseError, ParseResult};
    pub use crate::syntax::{Expression, FuncHeader, SyntaxTree};

    pub fn err<S: Into<String>, T>(message: S) -> ParseResult<T> {
        Err(ParseError::new(message))
    }
}

pub use align::AlignFunc;
pub use styles::{BoldFunc, ItalicFunc, MonospaceFunc};

/// Create a scope with all standard functions.
pub fn std() -> Scope {
    let mut std = Scope::new();
    std.add::<BoldFunc>("bold");
    std.add::<ItalicFunc>("italic");
    std.add::<MonospaceFunc>("mono");
    std.add::<AlignFunc>("align");
    std
}