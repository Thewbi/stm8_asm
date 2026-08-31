use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub static AST_NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);