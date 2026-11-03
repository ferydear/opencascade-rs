use std::{env::var, path::Path};

fn main() {
    // OCCT 源码目录（本 crate 的 OCCT 子模块；M19 起 = V8_0_1）
    println!(
        "cargo:rustc-env=OCCT_SRC_DIR={}",
        Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("OCCT").to_string_lossy()
    );
    // M19：OCCT_PATCH_DIR 不再导出——8.0 移除了 BUILD_PATCH（无替代）
}
