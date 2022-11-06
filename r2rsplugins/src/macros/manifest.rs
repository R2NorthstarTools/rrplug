extern crate windres;

use windres::Build;

// #[macro_export]
macro_rules! include_manifest {
    () => {
        use windres::Build;

        Build::new().compile("headers/Resource.rc").unwrap();
    };
}
