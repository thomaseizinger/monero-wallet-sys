use autocxx::include_cpp;

include_cpp! {
    // C++ headers we want to include.

    #include "wallet/wallet2.h"

    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe)
    // What types and functions we want to generate
    generate!("tools::wallet2")
}

fn main() {}
