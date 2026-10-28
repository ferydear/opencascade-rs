#include <BRepOffsetAPI_MakeOffset.hxx>
#include <BRepOffsetAPI_MakePipe.hxx>
#include <BRepOffsetAPI_MakePipeShell.hxx>
#include <BRepOffsetAPI_MakeThickSolid.hxx>
#include <BRepOffsetAPI_ThruSections.hxx>
#include <Law_Function.hxx>
#include <TopTools_ListOfShape.hxx>
#include <TopoDS_Shape.hxx>
#include <bindings_common.hxx>

// On MSVC, Handle_Law_Function is a subclass of opencascade::handle<Law_Function>,
// so cxxbridge's member-function-pointer codegen fails (C2440). Forward through
// this free-function wrapper instead.
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows without /utf-8.
inline void BRepOffsetAPI_MakePipeShell_SetLaw(BRepOffsetAPI_MakePipeShell &make_pipe_shell,
                                               const TopoDS_Shape &profile,
                                               const Handle_Law_Function &law,
                                               bool with_contact,
                                               bool with_correction) {
  make_pipe_shell.SetLaw(profile, law, with_contact, with_correction);
}
