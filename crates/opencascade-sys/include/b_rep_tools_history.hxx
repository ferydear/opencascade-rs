#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepTools_History.hxx>
#include <TopoDS_Shape.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <TopTools_ListOfShape.hxx>
#include <bindings_common.hxx>
#include <memory>

// On MSVC, Handle_BRepTools_History is a subclass of opencascade::handle<BRepTools_History>.
// All wrappers below take the Handle_ type directly, so cxxbridge's function-pointer
// codegen resolves to exact signatures; subclass-to-base conversions happen inside.
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows without /utf-8.

// BRepAlgoAPI_BuilderAlgo::History() returns the algorithm's own history
// (inherited by BRepAlgoAPI_Cut/Fuse/Common).
inline std::unique_ptr<Handle_BRepTools_History> BRepAlgoAPI_Cut_History(BRepAlgoAPI_Cut &cut) {
  return std::unique_ptr<Handle_BRepTools_History>(new Handle_BRepTools_History(cut.History()));
}

// BRepFilletAPI_MakeFillet (a BRepBuilderAPI_MakeShape) has no History() getter.
// The BRepTools_History(arguments, algo) template constructor maps every subshape
// of the arguments through the algo's IsDeleted/Modified/Generated.
inline std::unique_ptr<Handle_BRepTools_History> BRepFilletAPI_MakeFillet_History(
    BRepFilletAPI_MakeFillet &make_fillet, const TopoDS_Shape &shape_before) {
  TopTools_ListOfShape arguments;
  arguments.Append(shape_before);
  return std::unique_ptr<Handle_BRepTools_History>(
      new Handle_BRepTools_History(new BRepTools_History(arguments, make_fillet)));
}

inline std::unique_ptr<Handle_TopTools_HSequenceOfShape> History_Generated(
    const Handle_BRepTools_History &history, const TopoDS_Shape &shape) {
  TopTools_HSequenceOfShape *sequence = new TopTools_HSequenceOfShape();
  const TopTools_ListOfShape &list = history->Generated(shape);
  for (TopTools_ListOfShape::const_iterator it = list.begin(); it != list.end(); ++it) {
    sequence->Append(*it);
  }
  return std::unique_ptr<Handle_TopTools_HSequenceOfShape>(new Handle_TopTools_HSequenceOfShape(sequence));
}

inline std::unique_ptr<Handle_TopTools_HSequenceOfShape> History_Modified(
    const Handle_BRepTools_History &history, const TopoDS_Shape &shape) {
  TopTools_HSequenceOfShape *sequence = new TopTools_HSequenceOfShape();
  const TopTools_ListOfShape &list = history->Modified(shape);
  for (TopTools_ListOfShape::const_iterator it = list.begin(); it != list.end(); ++it) {
    sequence->Append(*it);
  }
  return std::unique_ptr<Handle_TopTools_HSequenceOfShape>(new Handle_TopTools_HSequenceOfShape(sequence));
}

inline bool History_IsRemoved(const Handle_BRepTools_History &history, const TopoDS_Shape &shape) {
  return history->IsRemoved(shape);
}

inline bool History_HasGenerated(const Handle_BRepTools_History &history) {
  return history->HasGenerated();
}

inline bool History_HasModified(const Handle_BRepTools_History &history) {
  return history->HasModified();
}

inline bool History_HasRemoved(const Handle_BRepTools_History &history) {
  return history->HasRemoved();
}

inline std::unique_ptr<Handle_BRepTools_History> BRepAlgoAPI_Fuse_History(BRepAlgoAPI_Fuse &fuse) {
  return std::unique_ptr<Handle_BRepTools_History>(new Handle_BRepTools_History(fuse.History()));
}

inline std::unique_ptr<Handle_BRepTools_History> BRepAlgoAPI_Common_History(BRepAlgoAPI_Common &common) {
  return std::unique_ptr<Handle_BRepTools_History>(new Handle_BRepTools_History(common.History()));
}

// Subshape identity for ID-evolution bookkeeping (same TShape + Location).
inline bool ShapesAreSame(const TopoDS_Shape &a, const TopoDS_Shape &b) {
  return a.IsSame(b);
}
