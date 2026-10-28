pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/shape_analysis.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type Handle_TopTools_HSequenceOfShape = crate::top_tools::Handle_TopTools_HSequenceOfShape;

        // MSVC 下 OCCT 的 Handle_ 是 opencascade::handle<> 的子类，cxxbridge 对静态成员
        // 取函数指针会因可变引用参数类型不匹配而 C2440，改为自由函数 + 手写包装
        // （include/shape_analysis.hxx）。
        pub fn ShapeAnalysis_FreeBounds_ConnectEdgesToWires(
            edges: Pin<&mut Handle_TopTools_HSequenceOfShape>,
            tolerance: f64,
            shared: bool,
            wires: Pin<&mut Handle_TopTools_HSequenceOfShape>,
        );
    }
}
