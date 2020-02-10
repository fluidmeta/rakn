use crate::core::package::Package;
use std::collections::HashMap;

pub trait Scanner {
    fn run(self) -> HashMap<String,Vec<Package>>;
}
