# ndi-sys

FFI bindings for the NDI

## Normal use

This lib tries various strategies to link against the NDI lib on different platforms. The crate attempts to dynamically link by default but can be forced to statically link by disabling default features.

## Generate bindings

If you want to regenerate the bindings, make sure that the `NDI_SDK_DIR` env variable to where you have the SDK and run with the `bindings` feature like this `cargo build --features bindings` which re-generates the sdk.rs file.


## License

All the NDI code is licensed under Newtek's license found in NDI-LICENSE.md

The binding code to generate rust stuff here is under MIT in LICENSE.md
