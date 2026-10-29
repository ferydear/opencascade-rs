//! m0_validation - M0 checklist items 1.3-1.6 in one run:
//!
//!   [1.3] parametric model: box -> hole (boolean cut) -> fillet on the 4
//!         vertical edges; parameter changes must change the geometry hash.
//!   [1.4] determinism: N rebuilds with identical parameters -> identical
//!         quantized geometry hash (architecture doc 07 section 5:
//!         topology counts + quantized volume/area/centroid/vertex coords;
//!         no raw float bit patterns).
//!   [1.5] history mapping: BRepTools_History of the boolean cut and the
//!         fillet must provide input-subshape -> output-subshape maps.
//!   [1.6] STEP round-trip: export -> re-import -> identical hash.
//!
//! Output is ASCII-only (zh-CN consoles decode GBK; keep it safe).
//! Exit code 0 = all checks passed, 1 = at least one failure.

use clap::Parser;
use glam::{dvec3, DVec3};
use opencascade::primitives::{Edge, Shape};
use opencascade_sys as ffi;

/// Length of an HSequence result (HSequences are 1-based indexable).
fn seq_len(seq: &ffi::top_tools::Handle_TopTools_HSequenceOfShape) -> usize {
    ffi::top_tools::TopTools_HSequenceOfShape_length(seq) as usize
}

#[derive(Parser, Debug)]
#[command(
    name = "m0_validation",
    about = "M0 checks 1.3-1.6: parametric model, hash, history, STEP round-trip"
)]
struct Args {
    /// Number of rebuilds for the determinism check (1.4)
    #[arg(long, default_value_t = 5)]
    runs: u32,

    /// Hole radius in mm (1.3)
    #[arg(long, default_value_t = 4.0)]
    hole_radius: f64,

    /// Fillet radius in mm (1.3)
    #[arg(long, default_value_t = 1.5)]
    fillet_radius: f64,
}

#[derive(Clone, Copy)]
struct Params {
    length: f64,
    width: f64,
    height: f64,
    hole_r: f64,
    fillet_r: f64,
}

// ---------------------------------------------------------------------------
// model: box -> through hole -> fillet on the 4 vertical edges
// ---------------------------------------------------------------------------

fn box_and_cylinder(p: &Params) -> (Shape, Shape) {
    let base = Shape::box_from_corners(dvec3(0.0, 0.0, 0.0), dvec3(p.length, p.width, p.height));
    let cutter = Shape::cylinder(
        dvec3(p.length / 2.0, p.width / 2.0, -1.0),
        p.hole_r,
        dvec3(0.0, 0.0, 1.0),
        p.height + 2.0,
    );
    (base, cutter)
}

fn is_vertical(start: DVec3, end: DVec3) -> bool {
    (start.x - end.x).abs() < 1e-7
        && (start.y - end.y).abs() < 1e-7
        && (start.z - end.z).abs() > 1e-7
}

/// Deduplication key: an edge is identified by its quantized, order-normalized
/// endpoint pair. TopExp enumerates shared subshapes once per adjacent parent,
/// so the same edge shows up multiple times during exploration.
fn quantized_pair(a: (f64, f64, f64), b: (f64, f64, f64)) -> (i64, i64, i64, i64, i64, i64) {
    let q = |v: f64| (v * 1e7).round() as i64;
    let (a, b) = if (a.2, a.1, a.0) < (b.2, b.1, b.0) { (a, b) } else { (b, a) };
    (q(a.0), q(a.1), q(a.2), q(b.0), q(b.1), q(b.2))
}

/// Vertical edges on the box boundary (the 4 corner edges), excluding the
/// cylindrical hole seam (which is also a vertical straight line).
fn is_box_corner_vertical(start: DVec3, end: DVec3, p: &Params) -> bool {
    if !is_vertical(start, end) {
        return false;
    }
    let on_boundary = |v: f64, hi: f64| v.abs() < 1e-6 || (hi - v).abs() < 1e-6;
    on_boundary(start.x, p.length) || on_boundary(start.y, p.width)
}

fn build_model(p: &Params) -> Shape {
    let (base, cutter) = box_and_cylinder(p);
    let cut = base.subtract(&cutter);
    let mut seen = std::collections::BTreeSet::new();
    let mut vertical: Vec<Edge> = Vec::new();
    for e in cut.edges() {
        let (s, t) = (e.start_point(), e.end_point());
        if !is_box_corner_vertical(s, t, p) {
            continue;
        }
        if seen.insert(quantized_pair((s.x, s.y, s.z), (t.x, t.y, t.z))) {
            vertical.push(e);
        }
    }
    assert_eq!(vertical.len(), 4, "expected exactly 4 vertical box corner edges");
    cut.fillet_edges(p.fillet_r, vertical)
}

// ---------------------------------------------------------------------------
// quantized geometry hash (doc 07 section 5, M0-simplified: absolute
// quantization - coords 1e-7 mm, volume 1e-9 mm^3, area 1e-8 mm^2)
// ---------------------------------------------------------------------------

use ffi::top_abs::TopAbs_ShapeEnum;

const COORD_Q: f64 = 1e7;
const VOLUME_Q: f64 = 1e9;
const AREA_Q: f64 = 1e8;

struct Fnv1a64(u64);

impl Fnv1a64 {
    fn new() -> Self {
        Fnv1a64(0xcbf2_9ce4_8422_2325)
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    fn i64(&mut self, v: i64) {
        self.write(&v.to_le_bytes());
    }
    fn quantized(&mut self, x: f64, scale: f64) {
        self.i64((x * scale).round() as i64);
    }
}

#[derive(Debug, Clone, Copy)]
struct Metrics {
    volume: f64,
    area: f64,
    solids: i64,
    faces: i64,
    edges: i64,
    vertices: i64,
}

impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "solids={} faces={} edges={} vertices={} volume={:.6}mm3 area={:.6}mm2",
            self.solids, self.faces, self.edges, self.vertices, self.volume, self.area
        )
    }
}

fn count_subshapes(shape: &ffi::topo_ds::TopoDS_Shape, kind: TopAbs_ShapeEnum) -> i64 {
    let mut explorer = ffi::top_exp::TopExp_Explorer_new(shape, kind);
    let mut n = 0;
    while explorer.More() {
        n += 1;
        explorer.pin_mut().Next();
    }
    n
}

/// Owned copies of all subshapes of `shape` of the given kind
/// (so iteration does not hold borrows against the explorer).
fn owned_subshapes(
    shape: &ffi::topo_ds::TopoDS_Shape,
    kind: TopAbs_ShapeEnum,
) -> Vec<cxx::UniquePtr<ffi::topo_ds::TopoDS_Shape>> {
    let mut out = Vec::new();
    let mut explorer = ffi::top_exp::TopExp_Explorer_new(shape, kind);
    while explorer.More() {
        out.push(ffi::top_exp::ExplorerCurrentShape(&explorer));
        explorer.pin_mut().Next();
    }
    out
}

fn vertex_coords_quantized(shape: &ffi::topo_ds::TopoDS_Shape) -> Vec<[i64; 3]> {
    let mut out = Vec::new();
    for v in owned_subshapes(shape, TopAbs_ShapeEnum::TopAbs_VERTEX) {
        let pnt = ffi::b_rep::BRep_Tool_Pnt(ffi::topo_ds::TopoDS::Vertex(&v));
        out.push([
            (pnt.X() * COORD_Q).round() as i64,
            (pnt.Y() * COORD_Q).round() as i64,
            (pnt.Z() * COORD_Q).round() as i64,
        ]);
    }
    // deterministic order independent of traversal order
    out.sort();
    out
}

fn quantized_hash(shape: &ffi::topo_ds::TopoDS_Shape) -> (u64, Metrics) {
    let mut volume_props = ffi::g_prop::GProps_new();
    ffi::b_rep_g_prop::BRepGProp::VolumeProperties(
        shape,
        volume_props.pin_mut(),
        /* only_closed */ true,
        /* skip_shared */ false,
        /* use_triangulation */ false,
    );
    let volume = volume_props.Mass();
    let centre = ffi::g_prop::GProp_GProps_CentreOfMass(&volume_props);

    let mut surface_props = ffi::g_prop::GProps_new();
    ffi::b_rep_g_prop::BRepGProp::SurfaceProperties(
        shape,
        surface_props.pin_mut(),
        /* skip_shared */ false,
        /* use_triangulation */ false,
    );
    let area = surface_props.Mass();

    let solids = count_subshapes(shape, TopAbs_ShapeEnum::TopAbs_SOLID);
    let faces = count_subshapes(shape, TopAbs_ShapeEnum::TopAbs_FACE);
    let edges = count_subshapes(shape, TopAbs_ShapeEnum::TopAbs_EDGE);
    let vertices = vertex_coords_quantized(shape);

    let mut h = Fnv1a64::new();
    h.i64(solids);
    h.i64(faces);
    h.i64(edges);
    h.i64(vertices.len() as i64);
    for c in &vertices {
        h.i64(c[0]);
        h.i64(c[1]);
        h.i64(c[2]);
    }
    h.quantized(volume, VOLUME_Q);
    h.quantized(area, AREA_Q);
    h.quantized(centre.X(), COORD_Q);
    h.quantized(centre.Y(), COORD_Q);
    h.quantized(centre.Z(), COORD_Q);

    let metrics = Metrics { volume, area, solids, faces, edges, vertices: vertices.len() as i64 };
    (h.0, metrics)
}

fn model_hash(p: &Params) -> u64 {
    quantized_hash(build_model(p).inner_shape()).0
}

// ---------------------------------------------------------------------------
// 1.5 history mapping (doc 06 section 3: input subshape -> output subshape)
// ---------------------------------------------------------------------------

fn edge_endpoints(edge_shape: &ffi::topo_ds::TopoDS_Shape) -> [(f64, f64, f64); 2] {
    let vs = owned_subshapes(edge_shape, TopAbs_ShapeEnum::TopAbs_VERTEX);
    assert!(vs.len() >= 2, "edge must have 2 vertices");
    let to_xyz = |v: &cxx::UniquePtr<ffi::topo_ds::TopoDS_Shape>| {
        let pnt = ffi::b_rep::BRep_Tool_Pnt(ffi::topo_ds::TopoDS::Vertex(v));
        (pnt.X(), pnt.Y(), pnt.Z())
    };
    [to_xyz(&vs[0]), to_xyz(&vs[vs.len() - 1])]
}

fn vertical_sys_edges(
    shape: &ffi::topo_ds::TopoDS_Shape,
    p: &Params,
) -> Vec<cxx::UniquePtr<ffi::topo_ds::TopoDS_Shape>> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for e in owned_subshapes(shape, TopAbs_ShapeEnum::TopAbs_EDGE) {
        let [a, b] = edge_endpoints(&e);
        let vertical =
            (a.0 - b.0).abs() < 1e-7 && (a.1 - b.1).abs() < 1e-7 && (a.2 - b.2).abs() > 1e-7;
        if !vertical {
            continue;
        }
        let on_boundary = |v: f64, hi: f64| v.abs() < 1e-6 || (hi - v).abs() < 1e-6;
        if !(on_boundary(a.0, p.length) || on_boundary(a.1, p.width)) {
            continue; // cylindrical hole seam
        }
        if seen.insert(quantized_pair(a, b)) {
            out.push(e);
        }
    }
    out
}

fn check_history(p: &Params) -> bool {
    let (base, cutter) = box_and_cylinder(p);
    let range = ffi::message::Message_ProgressRange_new();

    // ---- boolean cut history ----
    let mut cut =
        ffi::b_rep_algo_api::BRepAlgoAPI_Cut_new(base.inner_shape(), cutter.inner_shape());
    cut.pin_mut().Build(&range);
    if !cut.IsDone() {
        println!("  cut.IsDone() == false");
        return false;
    }
    let history = ffi::b_rep_tools_history::BRepAlgoAPI_Cut_History(cut.pin_mut());

    let box_faces = owned_subshapes(base.inner_shape(), TopAbs_ShapeEnum::TopAbs_FACE);
    let cutter_faces = owned_subshapes(cutter.inner_shape(), TopAbs_ShapeEnum::TopAbs_FACE);

    let mut faces_modified = 0;
    let mut faces_removed = 0;
    for f in &box_faces {
        if ffi::b_rep_tools_history::History_IsRemoved(&history, f) {
            faces_removed += 1;
        } else if seq_len(&ffi::b_rep_tools_history::History_Modified(&history, f)) > 0 {
            faces_modified += 1;
        }
    }
    let mut hole_faces_generated = 0;
    for f in &cutter_faces {
        hole_faces_generated += seq_len(&ffi::b_rep_tools_history::History_Generated(&history, f));
    }

    println!(
        "  cut: box faces={} modified={} removed={}; hole faces generated={} (has_mod={} has_gen={} has_rm={})",
        box_faces.len(),
        faces_modified,
        faces_removed,
        hole_faces_generated,
        ffi::b_rep_tools_history::History_HasModified(&history),
        ffi::b_rep_tools_history::History_HasGenerated(&history),
        ffi::b_rep_tools_history::History_HasRemoved(&history),
    );

    let cut_ok = faces_modified >= 1 && hole_faces_generated >= 1;

    // ---- fillet history ----
    let cut_inner: &ffi::topo_ds::TopoDS_Shape = cut.pin_mut().Shape();
    let vertical = vertical_sys_edges(cut_inner, p);
    if vertical.len() != 4 {
        println!("  fillet: expected 4 vertical edges, got {}", vertical.len());
        return false;
    }

    let mut make_fillet = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(cut_inner);
    for e in &vertical {
        make_fillet.pin_mut().add_edge(p.fillet_r, ffi::topo_ds::TopoDS::Edge(e));
    }
    make_fillet.pin_mut().Build(&range);
    if !make_fillet.IsDone() {
        println!("  fillet.IsDone() == false");
        return false;
    }

    let fillet_history = ffi::b_rep_tools_history::BRepFilletAPI_MakeFillet_History(
        make_fillet.pin_mut(),
        cut_inner,
    );

    let mut edges_with_fillet_face = 0;
    let mut edges_removed = 0;
    for e in &vertical {
        if ffi::b_rep_tools_history::History_IsRemoved(&fillet_history, e) {
            edges_removed += 1;
        }
        if seq_len(&ffi::b_rep_tools_history::History_Generated(&fillet_history, e)) > 0 {
            edges_with_fillet_face += 1;
        }
    }

    // original vertical faces adjacent to the filleted edges should be modified
    let mut faces_modified_by_fillet = 0;
    for f in &box_faces {
        if seq_len(&ffi::b_rep_tools_history::History_Modified(&fillet_history, f)) > 0 {
            faces_modified_by_fillet += 1;
        }
    }

    println!(
        "  fillet: vertical edges={} -> fillet faces on {} edges, removed edges={}; original faces modified={}",
        vertical.len(),
        edges_with_fillet_face,
        edges_removed,
        faces_modified_by_fillet
    );

    let fillet_ok = edges_with_fillet_face == 4;

    cut_ok && fillet_ok
}

// ---------------------------------------------------------------------------

fn main() {
    let args = Args::parse();
    let p = Params {
        length: 30.0,
        width: 20.0,
        height: 10.0,
        hole_r: args.hole_radius,
        fillet_r: args.fillet_radius,
    };

    println!(
        "m0_validation: box {}x{}x{} mm, hole r={} mm, fillet r={} mm",
        p.length, p.width, p.height, p.hole_r, p.fillet_r
    );

    let mut all_ok = true;

    // [1.3] build + metrics
    let model = build_model(&p);
    let (hash_base, metrics) = quantized_hash(model.inner_shape());
    println!("[1.3] model metrics: {}", metrics);
    println!("[1.3] hash = {:#018x}", hash_base);
    all_ok &= metrics.solids >= 1 && metrics.volume > 0.0;

    // [1.4] determinism: same params, N rebuilds
    let mut hashes = vec![hash_base];
    for _ in 1..args.runs {
        hashes.push(model_hash(&p));
    }
    let deterministic = hashes.iter().all(|h| *h == hash_base);
    println!(
        "[1.4] {} rebuilds identical hash: {} ({})",
        args.runs,
        deterministic,
        hashes.iter().map(|h| format!("{h:#018x}")).collect::<Vec<_>>().join(" ")
    );
    all_ok &= deterministic;

    // [1.3] param sensitivity: change one param at a time
    let mut p_fillet = p;
    p_fillet.fillet_r += 0.25;
    let mut p_hole = p;
    p_hole.hole_r += 0.5;
    let h_fillet = model_hash(&p_fillet);
    let h_hole = model_hash(&p_hole);
    let sensitive = h_fillet != hash_base && h_hole != hash_base;
    println!(
        "[1.3] param change alters hash: fillet+0.25mm={} hole_r+0.5mm={}",
        h_fillet != hash_base,
        h_hole != hash_base
    );
    all_ok &= sensitive;

    // [1.5] history
    let history_ok = check_history(&p);
    println!("[1.5] history mapping (cut + fillet): {}", if history_ok { "OK" } else { "FAIL" });
    all_ok &= history_ok;

    // [1.6] STEP round-trip
    let step_path = std::env::temp_dir().join("m0_roundtrip.step");
    let roundtrip_ok = match model.write_step(&step_path) {
        Ok(()) => match Shape::read_step(&step_path) {
            Ok(read_back) => {
                let (hash_back, metrics_back) = quantized_hash(read_back.inner_shape());
                let equal = hash_back == hash_base;
                println!(
                    "[1.6] STEP round-trip: {} (orig {:#018x} vs read {:#018x})",
                    if equal { "OK" } else { "MISMATCH" },
                    hash_base,
                    hash_back
                );
                if !equal {
                    println!("      orig : {}", metrics);
                    println!("      read : {}", metrics_back);
                }
                equal
            },
            Err(e) => {
                println!("[1.6] STEP read failed: {e}");
                false
            },
        },
        Err(e) => {
            println!("[1.6] STEP write failed: {e}");
            false
        },
    };
    all_ok &= roundtrip_ok;

    println!("RESULT: {}", if all_ok { "PASS" } else { "FAIL" });
    std::process::exit(if all_ok { 0 } else { 1 });
}
