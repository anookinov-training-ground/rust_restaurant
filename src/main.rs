use std::collections::HashMap; // idiomatic way to bring structs, enums, and other items with use

// exception to this idiom if 2 items with the same name are brought into scope with use statements
use std::fmt;
use std::io;

use std::fmt::Result;
use std::io::Result as IoResult; // provides new names with the as keyward

// --snip--
use std::cmp::Ordering;
use std::io;
use std::io::Write;

use std::{cmp::Ordering, io}; // using nested paths to clean up large use lists
use std::io::{self, Write};
// --snip--

use std::collections::*; // Glob operator to bring all public items defined in a path into scope

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

fn function3() -> Result {
    // --snip--
}

fn function4() -> IoResult<()> {
    // --snip--
}