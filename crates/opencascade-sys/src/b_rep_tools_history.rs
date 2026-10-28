pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_tools_history.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type Handle_TopTools_HSequenceOfShape = crate::top_tools::Handle_TopTools_HSequenceOfShape;
        type BRepAlgoAPI_Cut = crate::b_rep_algo_api::BRepAlgoAPI_Cut;
        type BRepFilletAPI_MakeFillet = crate::b_rep_fillet_api::BRepFilletAPI_MakeFillet;

        // Handles
        type Handle_BRepTools_History;
        pub fn IsNull(self: &Handle_BRepTools_History) -> bool;
        // End Handles

        // Boolean cut: extract the algorithm's own history
        // (BRepAlgoAPI_BuilderAlgo::History(), inherited by Cut/Fuse/Common).
        pub fn BRepAlgoAPI_Cut_History(
            cut: Pin<&mut BRepAlgoAPI_Cut>,
        ) -> UniquePtr<Handle_BRepTools_History>;

        // Fillet: BRepFilletAPI_MakeFillet has no History() getter, so build one
        // from the algorithm via the BRepTools_History(arguments, algo) constructor.
        pub fn BRepFilletAPI_MakeFillet_History(
            make_fillet: Pin<&mut BRepFilletAPI_MakeFillet>,
            shape_before: &TopoDS_Shape,
        ) -> UniquePtr<Handle_BRepTools_History>;

        // Queries (input subshape -> output subshapes).
        // Results come back in an HSequence (indexable, 1-based) because
        // CxxVector<T> does not support extern C++ element types.
        pub fn History_Generated(
            history: &Handle_BRepTools_History,
            shape: &TopoDS_Shape,
        ) -> UniquePtr<Handle_TopTools_HSequenceOfShape>;
        pub fn History_Modified(
            history: &Handle_BRepTools_History,
            shape: &TopoDS_Shape,
        ) -> UniquePtr<Handle_TopTools_HSequenceOfShape>;
        pub fn History_IsRemoved(history: &Handle_BRepTools_History, shape: &TopoDS_Shape) -> bool;
        pub fn History_HasGenerated(history: &Handle_BRepTools_History) -> bool;
        pub fn History_HasModified(history: &Handle_BRepTools_History) -> bool;
        pub fn History_HasRemoved(history: &Handle_BRepTools_History) -> bool;
    }
}
