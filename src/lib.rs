extern crate cfg_if;
extern crate wasm_bindgen;

pub mod lexer;
pub mod regex;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub use lexer::instruction::Statement;
pub use lexer::Lexer;
pub use regex::Regex;
