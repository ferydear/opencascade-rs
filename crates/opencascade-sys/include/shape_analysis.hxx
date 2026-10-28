#include <ShapeAnalysis.hxx>
#include <ShapeAnalysis_FreeBounds.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <bindings_common.hxx>

// On MSVC, Handle_TopTools_HSequenceOfShape is a subclass of opencascade::handle<>,
// so cxxbridge's function-pointer codegen for the OCCT static member fails (C2440).
// Forward through this free-function wrapper instead (implicit subclass-to-base
// reference conversion; identical types on GCC/Clang).
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows without /utf-8.
inline void ShapeAnalysis_FreeBounds_ConnectEdgesToWires(Handle_TopTools_HSequenceOfShape &edges,
                                                         double tolerance,
                                                         bool shared,
                                                         Handle_TopTools_HSequenceOfShape &wires) {
  ShapeAnalysis_FreeBounds::ConnectEdgesToWires(edges, tolerance, shared, wires);
}
