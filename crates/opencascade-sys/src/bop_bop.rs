//! M42：BOPAlgo_BOP 多参批量布尔（GEO 2.4——单次 BOP 多 Argument/Tool）。

pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/bop_algo.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type Message_ProgressRange = crate::message::Message_ProgressRange;
        type BOPAlgo_Operation = crate::bop_algo::BOPAlgo_Operation;
        type BOPAlgo_GlueEnum = crate::bop_algo::BOPAlgo_GlueEnum;

        pub type BOPAlgo_BOP;
        #[cxx_name = "construct_unique_default"]
        pub fn BOPAlgo_BOP_new_empty() -> UniquePtr<BOPAlgo_BOP>;
        // 多参数（BOPAlgo_Builder 继承面）
        #[cxx_name = "AddArgument"]
        pub fn BOPAlgo_BOP_AddArgument(self: Pin<&mut BOPAlgo_BOP>, shape: &TopoDS_Shape);
        #[cxx_name = "AddTool"]
        pub fn BOPAlgo_BOP_AddTool(self: Pin<&mut BOPAlgo_BOP>, shape: &TopoDS_Shape);
        pub fn SetOperation(self: Pin<&mut BOPAlgo_BOP>, op: BOPAlgo_Operation);
        // BOPAlgo_Options 继承面：Glue 模式（相离体免相交——批量阵列的正解）
        pub fn SetGlue(self: Pin<&mut BOPAlgo_BOP>, glue: BOPAlgo_GlueEnum);
        pub fn Perform(self: Pin<&mut BOPAlgo_BOP>, progress: &Message_ProgressRange);
        // BOPAlgo_Options 继承面
        pub fn HasErrors(self: &BOPAlgo_BOP) -> bool;
        // BOPAlgo_BuilderShape 继承面
        #[cxx_name = "Shape"]
        pub fn BOPAlgo_BOP_Shape(self: &BOPAlgo_BOP) -> &TopoDS_Shape;
    }
}
