#include <TopTools_ListOfShape.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Builder.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <TopoDS_Shell.hxx>
#include <TopoDS_Solid.hxx>
#include <TopoDS_Vertex.hxx>
#include <bindings_common.hxx>

// astrakernel M19（OCCT 8.0.1）：TopoDS 从类改为命名空间（静态转型函数变自由函数），
// cxx 的 type+#[Self] 机制不可用——平名 shim 转发，桥侧改自由函数声明。
inline const TopoDS_Vertex &cast_shape_to_vertex(const TopoDS_Shape &shape) { return TopoDS::Vertex(shape); }
inline const TopoDS_Edge &cast_shape_to_edge(const TopoDS_Shape &shape) { return TopoDS::Edge(shape); }
inline const TopoDS_Wire &cast_shape_to_wire(const TopoDS_Shape &shape) { return TopoDS::Wire(shape); }
inline const TopoDS_Face &cast_shape_to_face(const TopoDS_Shape &shape) { return TopoDS::Face(shape); }
inline const TopoDS_Shell &cast_shape_to_shell(const TopoDS_Shape &shape) { return TopoDS::Shell(shape); }
inline const TopoDS_Solid &cast_shape_to_solid(const TopoDS_Shape &shape) { return TopoDS::Solid(shape); }
inline const TopoDS_Compound &cast_shape_to_compound(const TopoDS_Shape &shape) { return TopoDS::Compound(shape); }
