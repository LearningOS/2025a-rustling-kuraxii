// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// 当`unsafe`被用于标记项声明（如函数、特质等）时，它同时声明了一项契约。然而，该契约的内容无法仅通过单个关键字完全表达。因此，您有责任在项的文档注释中的`安全性(Safety)`部分手动阐明该契约。当`unsafe`被标记在由花括号包裹的代码块上时，它声明了对某些契约的遵守，例如某些指针参数的有效性或某些内存地址的所有权。但如同上文所述，您仍需在代码块的注释中说明是如何确保契约成立的。注意：所有注释都是为了提升代码的可读性和可维护性，而Rust编译器将代码正确性的信任完全交给了您！若您无法自行证明代码的内存安全性和正确性，请退一步改用安全代码！
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
