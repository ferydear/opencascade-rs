use std::{
    env::var,
    path::{Path, PathBuf},
};

const LIB_DIR: &str = "lib";
const INCLUDE_DIR: &str = "include";

/// Get the path to the OCCT library installation directory to be
/// used in build scripts.
///
/// Only valid during build (`cargo clean` removes these files).
pub fn occt_path() -> PathBuf {
    // moves the output into target/TARGET/OCCT
    // this way its less likely to be rebuilt without a cargo clean
    Path::new(&var("OUT_DIR").expect("missing OUT_DIR")).join("../../../../OCCT")
}

/// Build the OCCT library.
///
/// astrakernel M19（OCCT 8.0.1，2026-11-03）：
/// - **BUILD_PATCH 已被 8.0 移除**（升级指南 upgrade_occt800_minimums）——
///   上游 7.8.1 的 patch/adm/MODULES 裁剪方式不再可用。
/// - 改用 BUILD_MODULE 开关：Visualization/Draw 关闭；ApplicationFramework
///   **保持默认开**（TKCAF/TKLCAF/TKXCAF 是我们的链接面；8.0 默认 AF 模块
///   工具包列表已是最小集，无需 patch 重定义）。
/// - 8.0 源码树重排为 src/<Module>/<Toolkit>/<Package>/，C++17 基线，均由
///   OCCT 自身 cmake 处理；本函数只管模块开关与安装布局。
pub fn build_occt() {
    cmake::Config::new(Path::new(env!("OCCT_SRC_DIR")))
        .define("BUILD_LIBRARY_TYPE", "Static")
        .define("BUILD_MODULE_Draw", "FALSE")
        .define("BUILD_MODULE_Visualization", "FALSE")
        .define("USE_D3D", "FALSE")
        .define("USE_DRACO", "FALSE")
        .define("USE_EIGEN", "FALSE")
        .define("USE_FFMPEG", "FALSE")
        .define("USE_FREEIMAGE", "FALSE")
        .define("USE_FREETYPE", "FALSE")
        .define("USE_GLES2", "FALSE")
        .define("USE_OPENGL", "FALSE")
        .define("USE_RAPIDJSON", "FALSE")
        .define("USE_TBB", "FALSE")
        .define("USE_TCL", "FALSE")
        .define("USE_TK", "FALSE")
        .define("USE_VTK", "FALSE")
        .define("USE_XLIB", "FALSE")
        .define("INSTALL_DIR_LIB", LIB_DIR)
        .define("INSTALL_DIR_INCLUDE", INCLUDE_DIR)
        .profile("Release")
        .out_dir(occt_path())
        .build();
}
