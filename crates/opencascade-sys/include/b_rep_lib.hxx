#include <BRepLib.hxx>
#include <BRepLib_ToolTriangulatedShape.hxx>
#include <Poly_Triangulation.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <bindings_common.hxx>

// On MSVC, Handle_Poly_Triangulation is a subclass of opencascade::handle<Poly_Triangulation>,
// so cxxbridge cannot take the address of the OCCT static member directly. Forward through
// this free-function wrapper (implicit subclass-to-base reference conversion; identical
// types on GCC/Clang).
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows without /utf-8.
inline void BRepLib_ToolTriangulatedShape_ComputeNormals(const TopoDS_Face &face,
                                                         const Handle_Poly_Triangulation &triangulation) {
  BRepLib_ToolTriangulatedShape::ComputeNormals(face, triangulation);
}
