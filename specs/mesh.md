# Mesh Deformation Types

Adds support for four mesh-related Rive object types used for image warping and distortion via triangle-mesh deformation.

## Background

Mesh deformation allows images in Rive to be warped by overlaying a triangle mesh on top of them. The mesh is defined by vertices with UV coordinates that map into the image texture. The runtime triangulates the mesh boundary (contour vertices) and interior vertices, then renders the image through the deformed triangle mesh. Forced edges constrain the triangulation by requiring specific edges between vertices.

**Hierarchy**: `Image` -> `Mesh` -> `MeshVertex` / `ContourMeshVertex` / `ForcedEdge`

A Mesh is always a child of an Image. MeshVertex, ContourMeshVertex, and ForcedEdge are always children of a Mesh.

---

## Type 1: MeshVertex (typeKey: 108)

### Description

A vertex in a mesh deformation grid. Each MeshVertex defines a point in the mesh with UV texture coordinates that map to a position in the parent image. During deformation, the vertex position (inherited from Vertex via x/y) can be animated while the UV coordinates remain fixed, causing the image to warp.

### Inheritance Chain (C++ runtime)

`Component` -> `Vertex` -> `MeshVertex`

### Properties

| Property | Key ID | Type | Required | Default | Description |
|----------|--------|------|----------|---------|-------------|
| name | 4 (COMPONENT_NAME) | String | No | "" | Component name |
| parentId | 5 (COMPONENT_PARENT_ID) | UInt | Yes | - | Parent Mesh object index (artboard-local) |
| x | 24 (VERTEX_X) | Float | No | 0.0 | Vertex X position in artboard space |
| y | 25 (VERTEX_Y) | Float | No | 0.0 | Vertex Y position in artboard space |
| u | 215 | Float | No | 0.0 | U texture coordinate (0.0-1.0, maps horizontally into image) |
| v | 216 | Float | No | 0.0 | V texture coordinate (0.0-1.0, maps vertically into image) |

### Notes

- MeshVertex inherits from Vertex (typeKey 107, abstract) which provides x/y properties
- UV coordinates define where this vertex samples from the source image texture
- Interior vertices (not on the boundary) - used to add detail to the mesh grid
- Can be animated via KeyFrameDouble on x, y, u, or v properties

---

## Type 2: Mesh (typeKey: 109)

### Description

A mesh deformation container attached to an Image. Defines a triangle mesh overlay that allows the image to be warped/distorted by moving mesh vertices. The Mesh object itself stores no geometry -- its children (MeshVertex, ContourMeshVertex, ForcedEdge) define the mesh structure. The runtime performs Delaunay triangulation on the vertices to create the render mesh.

### Inheritance Chain (C++ runtime)

`Component` -> `ContainerComponent` -> `Mesh`

### Properties

| Property | Key ID | Type | Required | Default | Description |
|----------|--------|------|----------|---------|-------------|
| name | 4 (COMPONENT_NAME) | String | No | "" | Component name |
| parentId | 5 (COMPONENT_PARENT_ID) | UInt | Yes | - | Parent Image object index (artboard-local) |

### Notes

- Mesh must be a child of an Image (typeKey 100) -- attaching to other types is undefined behavior
- A Mesh has no properties of its own beyond the inherited Component name/parentId
- The mesh structure is defined entirely by its children: ContourMeshVertex (boundary), MeshVertex (interior), ForcedEdge (constrained edges)
- ContourMeshVertex children define the boundary polygon; MeshVertex children add interior detail points
- At minimum, a mesh needs 3+ ContourMeshVertex children to form a valid boundary triangle

---

## Type 3: ContourMeshVertex (typeKey: 111)

### Description

A vertex on the boundary (contour) of a mesh. ContourMeshVertex objects define the outer polygon of the mesh. The runtime connects these in order to form the mesh boundary, then triangulates the interior. Like MeshVertex, each ContourMeshVertex has UV coordinates mapping into the parent image.

### Inheritance Chain (C++ runtime)

`Component` -> `Vertex` -> `MeshVertex` -> `ContourMeshVertex`

### Properties

| Property | Key ID | Type | Required | Default | Description |
|----------|--------|------|----------|---------|-------------|
| name | 4 (COMPONENT_NAME) | String | No | "" | Component name |
| parentId | 5 (COMPONENT_PARENT_ID) | UInt | Yes | - | Parent Mesh object index (artboard-local) |
| x | 24 (VERTEX_X) | Float | No | 0.0 | Vertex X position in artboard space |
| y | 25 (VERTEX_Y) | Float | No | 0.0 | Vertex Y position in artboard space |
| u | 215 | Float | No | 0.0 | U texture coordinate (0.0-1.0) |
| v | 216 | Float | No | 0.0 | V texture coordinate (0.0-1.0) |

### Notes

- ContourMeshVertex extends MeshVertex -- it has the same properties but is semantically distinct
- Contour vertices are connected in sequence to form the mesh boundary polygon
- The boundary must be a simple (non-self-intersecting) polygon
- At least 3 contour vertices are needed for a valid mesh
- Order matters: contour vertices define the boundary winding order

---

## Type 4: ForcedEdge (typeKey: 112)

### Description

A forced edge constraint in mesh triangulation. Forces the Delaunay triangulator to include a specific edge between two vertices in the final triangle mesh. This is used to control the mesh topology -- without forced edges, the triangulator is free to choose any valid triangulation.

### Inheritance Chain (C++ runtime)

`Component` -> `ForcedEdge`

### Properties

| Property | Key ID | Type | Required | Default | Description |
|----------|--------|------|----------|---------|-------------|
| name | 4 (COMPONENT_NAME) | String | No | "" | Component name |
| parentId | 5 (COMPONENT_PARENT_ID) | UInt | Yes | - | Parent Mesh object index (artboard-local) |
| fromId | 219 | UInt | No | 0 | Source vertex index (artboard-local object index of a MeshVertex or ContourMeshVertex) |
| toId | 220 | UInt | No | 0 | Target vertex index (artboard-local object index of a MeshVertex or ContourMeshVertex) |

### Notes

- Both fromId and toId reference artboard-local indices of MeshVertex or ContourMeshVertex objects
- The referenced vertices must be children of the same parent Mesh
- Forced edges should not cross other forced edges or the mesh boundary

---

## Implementation Plan

### 1. Add type_key constants to `src/objects/core.rs`

Add to the `type_keys` module:

```rust
pub const MESH_VERTEX: u16 = 108;
pub const MESH: u16 = 109;
pub const CONTOUR_MESH_VERTEX: u16 = 111;
pub const FORCED_EDGE: u16 = 112;
```

### 2. Add property_key constants to `src/objects/core.rs`

Add to the `property_keys` module:

```rust
pub const MESH_VERTEX_U: u16 = 215;
pub const MESH_VERTEX_V: u16 = 216;
pub const FORCED_EDGE_FROM_ID: u16 = 219;
pub const FORCED_EDGE_TO_ID: u16 = 220;
```

### 3. Update `property_backing_type()` in `src/objects/core.rs`

- Add `MESH_VERTEX_U` and `MESH_VERTEX_V` to the Float arm
- Add `FORCED_EDGE_FROM_ID` and `FORCED_EDGE_TO_ID` to the UInt arm

### 4. Create struct implementations

Add to a new file `src/objects/mesh.rs` (or add to `src/objects/shapes.rs` if preferred):

```rust
pub struct MeshVertex {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

pub struct Mesh {
    pub name: String,
    pub parent_id: u64,
}

pub struct ContourMeshVertex {
    pub name: String,
    pub parent_id: u64,
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

pub struct ForcedEdge {
    pub name: String,
    pub parent_id: u64,
    pub from_id: u64,
    pub to_id: u64,
}
```

Each struct implements `RiveObject`. Only emit non-default properties (skip x/y/u/v when 0.0, skip fromId/toId when 0).

### 5. Add ObjectSpec variants to `src/builder/spec.rs`

```rust
Mesh {
    name: String,
    children: Option<Vec<ObjectSpec>>,
},
#[serde(rename = "mesh_vertex")]
MeshVertex {
    name: String,
    x: Option<f32>,
    y: Option<f32>,
    u: Option<f32>,
    v: Option<f32>,
},
#[serde(rename = "contour_mesh_vertex")]
ContourMeshVertex {
    name: String,
    x: Option<f32>,
    y: Option<f32>,
    u: Option<f32>,
    v: Option<f32>,
},
#[serde(rename = "forced_edge")]
ForcedEdge {
    name: String,
    from_vertex: Option<String>,
    to_vertex: Option<String>,
},
```

Note: `ForcedEdge` uses `from_vertex`/`to_vertex` as string names in JSON (resolved to artboard-local indices in the builder, like `bone` on Tendon or `target` on constraints).

### 6. Add `ParentKind::Mesh` to `src/builder/spec.rs`

Add `Mesh` variant to the `ParentKind` enum for child dispatch.

### 7. Handle in `append_object()` in `src/builder/objects.rs`

- `ObjectSpec::Mesh`: create Mesh object, recurse into children with `ParentKind::Mesh`
- `ObjectSpec::MeshVertex`: create MeshVertex, set x/y/u/v from options
- `ObjectSpec::ContourMeshVertex`: create ContourMeshVertex, set x/y/u/v from options
- `ObjectSpec::ForcedEdge`: create ForcedEdge, resolve `from_vertex`/`to_vertex` names to artboard-local indices via `name_to_index`

Also update `ObjectSpec::Image` to accept `children` so a Mesh can be nested under an Image.

### 8. Add test fixture at `tests/fixtures/mesh.json`

```json
{
  "scene_format_version": 1,
  "artboard": {
    "name": "MeshDemo",
    "width": 500,
    "height": 500,
    "children": [
      {
        "type": "image_asset",
        "name": "TestImage",
        "asset_id": 1
      },
      {
        "type": "image",
        "name": "WarpedImage",
        "asset_id": 1,
        "x": 250,
        "y": 250,
        "children": [
          {
            "type": "mesh",
            "name": "ImageMesh",
            "children": [
              {
                "type": "contour_mesh_vertex",
                "name": "TL",
                "x": 0, "y": 0,
                "u": 0.0, "v": 0.0
              },
              {
                "type": "contour_mesh_vertex",
                "name": "TR",
                "x": 200, "y": 0,
                "u": 1.0, "v": 0.0
              },
              {
                "type": "contour_mesh_vertex",
                "name": "BR",
                "x": 200, "y": 200,
                "u": 1.0, "v": 1.0
              },
              {
                "type": "contour_mesh_vertex",
                "name": "BL",
                "x": 0, "y": 200,
                "u": 0.0, "v": 1.0
              },
              {
                "type": "mesh_vertex",
                "name": "Center",
                "x": 100, "y": 100,
                "u": 0.5, "v": 0.5
              },
              {
                "type": "forced_edge",
                "name": "Diagonal",
                "from_vertex": "TL",
                "to_vertex": "BR"
              }
            ]
          }
        ]
      }
    ]
  }
}
```

---

## JSON Schema

### Mesh

```json
{
  "type": "mesh",
  "name": "<string>",
  "children": [
    { "type": "contour_mesh_vertex", "..." : "..." },
    { "type": "mesh_vertex", "..." : "..." },
    { "type": "forced_edge", "..." : "..." }
  ]
}
```

Parent: Must be a child of an `image` object.

### MeshVertex

```json
{
  "type": "mesh_vertex",
  "name": "<string>",
  "x": 0.0,
  "y": 0.0,
  "u": 0.0,
  "v": 0.0
}
```

Parent: Must be a child of a `mesh` object.

### ContourMeshVertex

```json
{
  "type": "contour_mesh_vertex",
  "name": "<string>",
  "x": 0.0,
  "y": 0.0,
  "u": 0.0,
  "v": 0.0
}
```

Parent: Must be a child of a `mesh` object.

### ForcedEdge

```json
{
  "type": "forced_edge",
  "name": "<string>",
  "from_vertex": "<vertex_name>",
  "to_vertex": "<vertex_name>"
}
```

Parent: Must be a child of a `mesh` object. `from_vertex` and `to_vertex` reference names of MeshVertex or ContourMeshVertex siblings.

---

## Acceptance Criteria

1. **Type keys match C++ runtime**: MeshVertex=108, Mesh=109, ContourMeshVertex=111, ForcedEdge=112
2. **Property keys match C++ runtime**: u=215, v=216, fromId=219, toId=220, plus inherited vertex x=24, y=25
3. **`property_backing_type()` updated**: u/v return Float; fromId/toId return UInt
4. **RiveObject implementations**: all four types implement the trait with correct type_key and properties
5. **Only non-default properties emitted**: x/y/u/v skip when 0.0; fromId/toId skip when 0
6. **Builder support**: `ObjectSpec` enum has variants for all four types; `append_object()` handles them
7. **Image gets children**: `ObjectSpec::Image` is extended with `children: Option<Vec<ObjectSpec>>` so Mesh can be nested under Image
8. **Name resolution**: ForcedEdge `from_vertex`/`to_vertex` are resolved from names to artboard-local indices (same pattern as Tendon bone resolution)
9. **Hierarchy enforced**: Mesh only valid under Image; MeshVertex/ContourMeshVertex/ForcedEdge only valid under Mesh
10. **Test fixture**: `tests/fixtures/mesh.json` generates a valid .riv that passes `cargo run -- validate`
11. **E2E test**: test case in `tests/e2e.rs` exercises generate + validate round-trip
12. **Unit tests**: each struct has tests for type_key, default properties, and non-default property emission (in `#[cfg(test)] mod tests` block)
13. **Generated registry unchanged**: types 108/109/111/112 and properties 215/216/219/220 already present in `generated_registry.rs`; no edits needed there
14. **Clippy and fmt pass**: `cargo clippy -- -D warnings` and `cargo fmt --check` clean
