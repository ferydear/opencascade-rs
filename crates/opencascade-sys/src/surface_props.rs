pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/surface_props.hxx");

        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type TopoDS_Edge = crate::topo_ds::TopoDS_Edge;
        type gp_Dir = crate::gp::gp_Dir;

        /// 面法向（平面=法向，圆柱/圆锥=轴向；其他类型 false）
        pub fn FaceNormal(face: &TopoDS_Face, out_dir: Pin<&mut gp_Dir>) -> bool;

        /// 面面积（mm²）
        pub fn FaceArea(face: &TopoDS_Face) -> f64;

        /// 边长度（mm）
        pub fn EdgeLength(edge: &TopoDS_Edge) -> f64;
    }
}
