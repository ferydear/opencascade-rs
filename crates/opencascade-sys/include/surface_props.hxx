#include <BRepAdaptor_Surface.hxx>
#include <BRepCheck_Analyzer.hxx>
#include <BRepGProp.hxx>
#include <BRepTools.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <GProp_GProps.hxx>
#include <gp_Dir.hxx>
#include <memory>

// Face/edge metrics for selector evaluation (kernel-geo).
// All free functions with exact signatures (cxxbridge function-pointer codegen).
// NOTE: keep this file ASCII-only; cl.exe assumes GBK on zh-CN Windows.

// Face normal: plane -> plane normal; cylinder/cone/torus -> axis.
// Returns false for other surface types.
inline bool FaceNormal(const TopoDS_Face &face, gp_Dir &out_dir) {
  BRepAdaptor_Surface surf(face, /* doRestrict */ true);
  switch (surf.GetType()) {
    case GeomAbs_Plane: {
      out_dir = surf.Plane().Axis().Direction();
      if (face.Orientation() == TopAbs_REVERSED) {
        out_dir.Reverse();
      }
      return true;
    }
    case GeomAbs_Cylinder: {
      out_dir = surf.Cylinder().Axis().Direction();
      return true;
    }
    case GeomAbs_Cone: {
      out_dir = surf.Cone().Axis().Direction();
      return true;
    }
    default:
      return false;
    }
}

inline double FaceArea(const TopoDS_Face &face) {
  GProp_GProps props;
  BRepGProp::SurfaceProperties(face, props);
  return props.Mass();
}

inline double EdgeLength(const TopoDS_Edge &edge) {
  GProp_GProps props;
  BRepGProp::LinearProperties(edge, props);
  return props.Mass();
}

// Whole-shape validity via BRepCheck_Analyzer (kernel-geo check_validity Standard/Full).
inline bool ShapeIsValid(const TopoDS_Shape &shape) {
  return BRepCheck_Analyzer(shape).IsValid();
}
