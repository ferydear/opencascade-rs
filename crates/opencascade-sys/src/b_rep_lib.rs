pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_lib.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type Handle_Poly_Triangulation = crate::poly::Handle_Poly_Triangulation;

        type BRepLib;
        #[Self = "BRepLib"]
        pub fn BuildCurves3d(shape: &TopoDS_Shape) -> bool;

        // MSVC 下 OCCT 的 Handle_Poly_Triangulation 是 opencascade::handle<> 的子类，
        // cxxbridge 对关联函数生成"取成员地址"的代码会因参数类型不匹配而 C2440，
        // 改为自由函数 + 手写包装（include/b_rep_lib.hxx）。
        pub fn BRepLib_ToolTriangulatedShape_ComputeNormals(
            face: &TopoDS_Face,
            triangulation: &Handle_Poly_Triangulation,
        );
    }
}
