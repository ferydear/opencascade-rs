#include <TopTools_HSequenceOfShape.hxx>
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx>
#include <TopTools_IndexedMapOfShape.hxx>
#include <TopTools_ListOfShape.hxx>
#include <TopoDS_Face.hxx>
#include <bindings_common.hxx>

// On MSVC, Handle_TopTools_HSequenceOfShape is a subclass of opencascade::handle<>.
// cxxbridge function-pointer codegen needs an exactly-matching overload
// (non-template beats the template in bindings_common.hxx, no ambiguity on GCC/Clang).
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows without /utf-8.
inline const TopTools_HSequenceOfShape &handle_try_deref(const Handle_TopTools_HSequenceOfShape &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}

inline std::unique_ptr<Handle_TopTools_HSequenceOfShape> new_Handle_TopTools_HSequenceOfShape() {
  auto sequence = new TopTools_HSequenceOfShape();
  auto handle = new Handle_TopTools_HSequenceOfShape(sequence);

  return std::unique_ptr<Handle_TopTools_HSequenceOfShape>(handle);
}

inline void TopTools_HSequenceOfShape_append(Handle_TopTools_HSequenceOfShape &handle, const TopoDS_Shape &shape) {
  handle->Append(shape);
}

inline Standard_Integer TopTools_HSequenceOfShape_length(const Handle_TopTools_HSequenceOfShape &handle) {
  return handle->Length();
}

inline const TopoDS_Shape &TopTools_HSequenceOfShape_value(const Handle_TopTools_HSequenceOfShape &handle,
                                                           Standard_Integer index) {
  return handle->Value(index);
}
