#include <Poly_Connect.hxx>
#include <Poly_Triangulation.hxx>
#include <bindings_common.hxx>

// On MSVC, Handle_Poly_Triangulation is a subclass of opencascade::handle<Poly_Triangulation>.
// cxxbridge function-pointer codegen needs an exactly-matching overload; the non-template
// wins over the function template in bindings_common.hxx (no ambiguity on GCC/Clang).
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows without /utf-8.
inline const Poly_Triangulation &handle_try_deref(const Handle_Poly_Triangulation &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}

inline std::unique_ptr<Handle_Poly_Triangulation>
Handle_Poly_Triangulation_new(std::unique_ptr<Poly_Triangulation> triangulation) {
  return std::unique_ptr<Handle_Poly_Triangulation>(new Handle_Poly_Triangulation(triangulation.release()));
}

inline std::unique_ptr<gp_Dir> Poly_Triangulation_Normal(const Poly_Triangulation &triangulation,
                                                         const Standard_Integer index) {
  return std::unique_ptr<gp_Dir>(new gp_Dir(triangulation.Normal(index)));
}

inline std::unique_ptr<gp_Pnt> Poly_Triangulation_Node(const Poly_Triangulation &triangulation,
                                                       const Standard_Integer index) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(triangulation.Node(index)));
}

inline std::unique_ptr<gp_Pnt2d> Poly_Triangulation_UV(const Poly_Triangulation &triangulation,
                                                       const Standard_Integer index) {
  return std::unique_ptr<gp_Pnt2d>(new gp_Pnt2d(triangulation.UVNode(index)));
}
