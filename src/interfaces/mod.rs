//! Contains logic for creating and managing interfaces

pub mod external;
pub mod interface;
pub mod manager;

#[cfg(test)]
mod test {
    use crate::rrplug;
    use rrplug_proc::as_interface;

    #[allow(dead_code)] // TODO: fix later
    #[repr(C)]
    struct TestInterface {
        the_line: &'static str,
    }

    #[as_interface]
    #[allow(improper_ctypes_definitions)]
    impl TestInterface {
        fn new() -> Self {
            Self { the_line: "line" }
        }

        pub const fn get_line(&self) -> &'static str {
            self.the_line
        }
    }

    // NOTE: this is not really super important
    // #[repr(C)]
    // struct GenericTestInterface<T> {
    //     smth: T,
    // }

    // #[as_interface]
    // #[allow(improper_ctypes_definitions)]
    // impl<T: Default + Clone + Sync + Send> GenericTestInterface<T> {
    //     fn new() -> Self {
    //         Self { smth: T::default() }
    //     }

    //     pub fn clone<T: Default + Clone + Sync + Send>(&self) -> T {
    //         self.smth.clone()
    //     }

    //     pub fn store(&self, smth: T) {
    //         _ = smth;
    //     }
    // }
}
