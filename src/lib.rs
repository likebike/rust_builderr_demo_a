extern {
    fn cfoo();
    fn cfoo2();
}

pub fn foo() {
    println!("Start of rust_builderr_demo_a::foo()");
    unsafe {
        cfoo();
        cfoo2();
    }
    println!("End of rust_builderr_demo_a::foo()");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        foo();
    }
}
