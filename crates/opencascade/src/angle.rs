// M18 基座统一（dev2/01 D3）：Angle/RVec/ToAngle 已上移 kernel-math
// （并增 Gon 变体）——本模块保留路径兼容，重导出 kernel-math 定义。
// fork 自持决策（2026-11-03）：不追上游，path-dep kernel-math。
pub use kernel_math::angle::{rvec, Angle, RVec, ToAngle};
