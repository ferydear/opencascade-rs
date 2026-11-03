// astrakernel M19（OCCT 8.0.1，2026-11-03）：桥接兼容层。
//
// OCCT 8.0 移除了各类的 Handle_* per-class typedef（升级指南
// upgrade_occt800_handle：`Handle(X)` → `occ::handle<>`；仅 Standard_Type.hxx
// 内保留了 Handle_Standard_Type 兼容 typedef）。本桥的 42 个 shim 头与 cxx
// 桥类型名（`type Handle_Xxx;`）仍使用旧名——在此集中恢复为别名，
// Rust 侧（.rs 桥声明）零改动。
//
// 注：opencascade::handle<T> 在 8.0.1 仍存在（occ::handle 为新增现代名）。

#pragma once

#include <BRepTools_History.hxx>
#include <Geom_Surface.hxx>
#include <Geom_Curve.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <Geom_Plane.hxx>
#include <Geom_BSplineCurve.hxx>
#include <Geom_BezierCurve.hxx>
#include <Geom_BezierSurface.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <Geom2d_Curve.hxx>
#include <Geom2d_TrimmedCurve.hxx>
#include <Geom2d_Ellipse.hxx>
#include <Law_Function.hxx>
#include <Poly_Triangulation.hxx>
#include <TColgp_HArray1OfPnt.hxx> // 8.0 弃用别名（src/Deprecated/NCollectionAliases），安装仍含
#include <TColgp_Array1OfPnt2d.hxx> // 同上
#include <TColgp_Array2OfPnt.hxx> // 同上
#include <TopTools_HSequenceOfShape.hxx> // 同上
#include <TopTools_ListOfShape.hxx> // 同上（8.0 起 OCCT 自家头不再传递引入，桥用到的名字在此显式补齐）
#include <TopTools_IndexedMapOfShape.hxx> // 同上
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx> // 同上

using Handle_BRepTools_History = opencascade::handle<BRepTools_History>;
using Handle_Geom_Surface = opencascade::handle<Geom_Surface>;
using Handle_Geom_Curve = opencascade::handle<Geom_Curve>;
using Handle_Geom_TrimmedCurve = opencascade::handle<Geom_TrimmedCurve>;
using Handle_Geom_Plane = opencascade::handle<Geom_Plane>;
using Handle_Geom_BSplineCurve = opencascade::handle<Geom_BSplineCurve>;
using Handle_Geom_BezierCurve = opencascade::handle<Geom_BezierCurve>;
using Handle_Geom_BezierSurface = opencascade::handle<Geom_BezierSurface>;
using Handle_Geom_CylindricalSurface = opencascade::handle<Geom_CylindricalSurface>;
using Handle_Geom2d_Curve = opencascade::handle<Geom2d_Curve>;
using Handle_Geom2d_TrimmedCurve = opencascade::handle<Geom2d_TrimmedCurve>;
using Handle_Geom2d_Ellipse = opencascade::handle<Geom2d_Ellipse>;
using Handle_Law_Function = opencascade::handle<Law_Function>;
using Handle_Poly_Triangulation = opencascade::handle<Poly_Triangulation>;
using Handle_TColgp_HArray1OfPnt = opencascade::handle<TColgp_HArray1OfPnt>;
using Handle_TopTools_HSequenceOfShape = opencascade::handle<TopTools_HSequenceOfShape>;
// Handle_Standard_Type：8.0.1 上游仍保留 typedef——不重复定义
