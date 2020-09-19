pub fn foo() {
    println!("Running rust_builderr_demo_a::foo()");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
