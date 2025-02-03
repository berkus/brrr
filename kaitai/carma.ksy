###
### BRender 1.1.2 data format
###
### Model ../DecodedData/DATA/MODELS/SCREWIE.DAT
### Actor ../DecodedData/DATA/ACTORS/SCREWIE.ACT
### Material ../DecodedData/DATA/MATERIAL/VLADCROM.MAT
### Pixelmap ../DecodedData/DATA/PIXELMAP/CHROME.PIX
###
meta:
  id: carmageddon_resource
  title: Carmageddon resource file definition
  application: Carmageddon 1 game data files
  endian: be
seq:
  - id: header
    type: chunk
  - id: chunks
    type: chunk
    repeat: eos
types:
  chunk_header:
    seq:
      - id: type
        type: u4
        enum: chunk_type
      - id: size
        type: u4
  chunk:
    seq:
      - id: header
        type: chunk_header
      - id: body
        #size: header.size - it's incorrect for several chunks, notably material_index
        type:
          switch-on: header.type
          cases:
            chunk_type::actor: actor_chunk
            chunk_type::actor_model: name_ref_chunk
            chunk_type::actor_material: actor_material_chunk
            chunk_type::actor_clip_plane: action_dummy_chunk
            chunk_type::actor_transform: action_dummy_chunk
            chunk_type::actor_light: action_dummy_chunk
            chunk_type::actor_camera: action_dummy_chunk
            chunk_type::actor_bounds: action_dummy_chunk
            chunk_type::actor_add_child: action_dummy_chunk
            chunk_type::file_info: file_info_chunk
            chunk_type::colour_map_ref: name_ref_chunk
            chunk_type::index_shade_ref: name_ref_chunk
            chunk_type::material_index: material_index_chunk
            chunk_type::vertices: vertices_chunk
            chunk_type::vertex_uv: vertex_uv_chunk
            chunk_type::face_material: face_material_chunk
            chunk_type::material: material_chunk
            chunk_type::pixels: pixels_chunk
            chunk_type::transform_matrix34: transform_matrix34_chunk
            chunk_type::transform_quat: transform_quat_chunk
            chunk_type::transform_euler: transform_euler_chunk
            chunk_type::transform_look_up: transform_look_up_chunk
            chunk_type::transform_translation: transform_translation_chunk
            chunk_type::faces: faces_chunk
            chunk_type::model: model_chunk
            chunk_type::pivot: pivot_chunk
            chunk_type::light: light_chunk
            chunk_type::bounds: bounds_chunk
            chunk_type::camera: camera_chunk
            chunk_type::pixelmap: pixelmap_chunk
  action_dummy_chunk: {}
  actor_chunk:
    seq:
      - id: type
        type: u1
        enum: actor_type
      - id: render_style
        type: u1
        enum: actor_render_style
      - id: identifier
        type: strz
        encoding: ascii
  name_ref_chunk:
    seq:
      - id: identifier
        type: strz
        encoding: ascii
  file_info_chunk:
    seq:
      - id: file_type
        type: u4
        enum: file_type
      - id: version
        type: u4
  pixelmap_chunk:
    seq:
      - id: type
        type: u1
        enum: pixelmap_type
      - id: row_bytes
        type: u2
      - id: width
        type: u2
      - id: height
        type: u2
      - id: origin_x
        type: u2
      - id: origin_y
        type: u2
      - id: identifier
        type: strz
        encoding: ascii
  pixels_chunk:
    seq:
      - id: units
        type: u4
      - id: unit_bytes
        type: u4
      - id: pixel_data
        size: payload_size
    instances:
      payload_size:
        value: units * unit_bytes
  model_chunk:
    seq:
      - id: flags # Only allow DONT_WELD, QUICK_UPDATE, KEEP_ORIGINAL and GENERATE_TAGS
        type: u2
      - id: identifier
        type: strz
        encoding: ascii
  pivot_chunk:
    seq:
      - id: pivot
        type: vec3f
  vertices_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: vertices
        type: vertex
        repeat: expr
        repeat-expr: entries_count
  vertex_uv_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: uvcoords
        type: vertex_uv
        repeat: expr
        repeat-expr: entries_count
  faces_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: faces
        type: face
        repeat: expr
        repeat-expr: entries_count
  material_index_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: materials
        type: strz
        encoding: ascii
        repeat: expr
        repeat-expr: entries_count
  face_material_chunk:
    seq:
      - id: per_face_material_index
        type: u2
        repeat: expr
        repeat-expr: _parent.header.size / 2
  vec2f:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
  vec3f:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
      - id: z
        type: f4
  vec4f:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
      - id: z
        type: f4
      - id: w
        type: f4
  vertex:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
      - id: z
        type: f4
  vertex_uv:
    seq:
      - id: u
        type: f4
      - id: v
        type: f4
  face:
    seq:
      - id: v1
        type: u2
      - id: v2
        type: u2
      - id: v3
        type: u2
      - id: smoothing
        type: u2
      - id: flags
        type: u1
  transform_matrix34_chunk:
    seq:
      - id: row0
        type: vec3f
      - id: row1
        type: vec3f
      - id: row2
        type: vec3f
      - id: row3_translate
        type: vec3f
  transform_quat_chunk:
    seq:
      - id: q_x
        type: f4
      - id: q_y
        type: f4
      - id: q_z
        type: f4
      - id: q_w
        type: f4
      - id: t
        type: vec3f
  transform_euler_chunk:
    seq:
      - id: angle_order
        type: u1
        enum: euler_angle_order
      - id: a
        type: f4
      - id: b
        type: f4
      - id: c
        type: f4
      - id: t
        type: vec3f
  transform_look_up_chunk:
    seq:
      - id: look
        type: vec3f
      - id: up
        type: vec3f
      - id: t
        type: vec3f
  transform_translation_chunk:
    seq:
      - id: t
        type: vec3f
  color:
    seq:
      - id: r
        type: u1
      - id: g
        type: u1
      - id: b
        type: u1
  material_chunk:
    seq:
      - id: colour
        type: color
      - id: opacity
        type: u1
      - id: ka
        type: f4
      - id: kd
        type: f4
      - id: ks
        type: f4
      - id: power
        type: f4
      - id: flags
        type: u2
      - id: map_transform_x
        type: vec2f
      - id: map_transform_y
        type: vec2f
      - id: map_transform_z
        type: vec2f
      - id: index_base
        type: u1
      - id: index_range
        type: u1
      - id: identifier
        type: strz
        encoding: ascii
  bounds_chunk:
    seq:
      - id: min
        type: vec3f
      - id: max
        type: vec3f
  plane_chunk:
    seq:
      - id: eqn
        type: vec4f
  light_chunk:
    seq:
      - id: type
        type: u1
        enum: light_type
      - id: colour
        type: color
      - id: attenuation_c
        type: f4
      - id: attenuation_l
        type: f4
      - id: attenuation_q
        type: f4
      - id: cone_inner
        type: f4
      - id: cone_outer
        type: f4
      - id: identifier
        type: strz
        encoding: ascii
  camera_chunk:
    seq:
      - id: type
        type: u1
        enum: camera_type
      - id: field_of_view
        type: f4
      - id: hither_z
        type: f4
      - id: yon_z
        type: f4
      - id: aspect
        type: f4
      - id: identifier
        type: strz
        encoding: ascii
enums:
  chunk_type:
    0x0: end                        # model load, actor load, material load
    0x3: pixelmap                   # CARMA uses BRender 1.1.2 apparently, not 1997
    0x4: material                   # material load
    0x9: old_material_index         # model load,
    0xa: old_vertices               # model load,
    0xb: old_vertices_uv            # model load,
    0xc: old_faces                  # model load,
    0xd: old_model                  # model load,
    0xe: add_model
    0xf: anim
    0x10: anim_transform
    0x11: anim_rate
    0x12: file_info                 # all load
    0x15: pivot                     # model load
    0x16: material_index            # model load, has_count
    0x17: vertices                  # model load, has_count
    0x18: vertex_uv                 # model load, has_count
    0x19: old_faces_1               # model load, has_count
    0x1a: face_material             # model load
    0x1b: old_model_1               # model load
    0x1c: colour_map_ref            # material load
    0x1e: index_blend_ref           # material load
    0x1f: index_shade_ref           # material load
    0x20: screendoor_ref            # material load
    0x21: pixels
    0x22: add_map                   # Connect a map to an indexed pixelmap
    0x23: actor                     # actor load,
    0x24: actor_model               # actor load,
    0x25: actor_transform           # actor load, (no chunk data)
    0x26: actor_material            # actor load,
    0x27: actor_light               # actor load, (no chunk data)
    0x28: actor_camera              # actor load, (no chunk data)
    0x29: actor_bounds              # actor load, (no chunk data)
    0x2a: actor_add_child           # actor load, (no chunk data)
    0x2b: transform_matrix34        # actor load,
    0x2c: transform_matrix34_lp     # actor load,
    0x2d: transform_quat            # actor load,
    0x2e: transform_euler           # actor load,
    0x2f: transform_look_up         # actor load,
    0x30: transform_translation     # actor load,
    0x31: transform_identity        # actor load, (no chunk data)
    0x32: bounds                    # actor load,
    0x33: light                     # actor load,
    0x34: camera                    # actor load,
    0x35: faces                     # model load, has_count
    0x36: model                     # model load,
    0x37: actor_clip_plane          # actor load, (no chunk data)
    0x38: plane                     # actor load,
    0x39: saturn_faces
    0x3a: saturn_model
  file_type:
    0x0: none
    0x1: actors
    0x2: pixelmap
    0x3: light
    0x4: camera
    0x5: material
    0xface: model
    0x0a11: animation
    0x5eed: tree
  actor_type:
    0x0: none
    0x1: model
    0x2: light
    0x3: camera
    0x4: reserved_
    0x5: bounds
    0x6: bounds_correct
    0x7: clip_plane
  actor_render_style:
    0x0: default
    0x1: none
    0x2: points
    0x3: edges
    0x4: faces
    0x5: bounding_points
    0x6: bounding_edges
    0x7: bounding_faces
  euler_angle_order:
    0x0: xyz_s
    0x1: xyx_s
    0x2: xzy_s
    0x3: xzx_s
    0x4: yzx_s
    0x5: yzy_s
    0x6: yxz_s
    0x7: yxy_s
    0x8: zxy_s
    0x9: zxz_s
    0xa: zyx_s
    0xb: zyz_s
    0xc: zyx_r
    0xd: xyx_r
    0xe: yzx_r
    0xf: xzx_r
    0x10: xzy_r
    0x11: yzy_r
    0x12: zxy_r
    0x13: yxy_r
    0x14: yxz_r
    0x15: zxz_r
    0x16: xyz_r
    0x17: zyz_r
  light_type:
    0x0: point
    0x1: direct
    0x2: spot
    0x4: view_point
    0x5: view_direct
    0x6: view_spot
  camera_type:
    0x0: parallel
    0x1: perspective
  pixelmap_type:
    0x0: index_1
    0x1: index_2
    0x2: index_4
    0x3: index_8
    0x4: rgb_555
    0x5: rgb_565
    0x6: rgb_888
    0x7: rgbx_888
    0x8: rgba_888
    0x9: yuyv_8888
    0xa: yuv_888
    0xb: depth_16
    0xc: depth_32
    0xd: alpha_8
    0xe: indexa_88
