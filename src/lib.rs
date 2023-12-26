// this panic macro just calls the panic! macro from the standard library
#[macro_export]
macro_rules! panic {
    ($($x:expr),*) => {{
        println!("Hey Look Ma, I Made It!");
        $(
           // call std panic!
              std::panic!($x);
        )*
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Check that the panic macro works
        panic!("This is a test");
    }
}
