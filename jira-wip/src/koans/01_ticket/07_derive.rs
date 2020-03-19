mod derive {
    /// Cool, we learned what a trait is and how to implement one.
    /// I am sure you agree with us though: implementing PartialEq was quite tedious
    /// and repetitive, a computer can surely do a better job without having to trouble us!
    /// 
    /// The Rust team feels your pain, hence a handy feature: derive macros.
    /// Derive macros are a code-generation tool: before code compilation kicks-in, derive
    /// macros take as input the code they have been applied to (as a stream of tokens!) 
    /// and they have a chance to generate other code, as needed.
    /// 
    /// For example, this `#[derive(PartialEq)]` will take as input the definition of our
    /// enum and generate an implementation of PartialEq which is exactly equivalent to
    /// the one we rolled out manually in the previous koan.
    /// You can check the code generated by derive macros using cargo expand: https://github.com/dtolnay/cargo-expand
    ///
    /// PartialEq is not the only trait whose implementation can be derived automatically!
    #[derive(PartialEq, Debug)]
    pub enum Status {
        ToDo,
        InProgress,
        Blocked,
        Done
    }

    /*
    #[derive(PartialEq)]
    pub enum Status {
        ToDo,
        InProgress,
        Blocked,
        Done
    }
    */

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn assertions() {
            // Your goal is to make this test compile.
            assert_eq!(Status::ToDo, Status::ToDo);
            assert_ne!(Status::Done, Status::ToDo);
            assert_ne!(Status::InProgress, Status::ToDo);
            assert_eq!(Status::InProgress, Status::InProgress);
        }
    }
}