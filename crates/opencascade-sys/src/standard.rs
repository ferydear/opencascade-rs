pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/standard.hxx");

        type Handle_Standard_Type;
        pub fn type_name(handle: &Handle_Standard_Type) -> String;
        pub fn IsNull(self: &Handle_Standard_Type) -> bool;
        // astrakernel M19（QUAL 10.1）：运行时 OCCT 版本（如 "8.0.1"）
        pub fn occt_version_string() -> String;
    }
}
