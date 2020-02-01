extern crate module_sample;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

use a::series::of;

fn main() {
    module_sample::client::connect();

    of::nested_modules();
}
