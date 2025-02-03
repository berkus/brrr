meta:
  id: brender_datafile
  file-extension: dat
  endian: be
  bit-endian: be

doc-ref: fw/formats.h
doc: |
  This is a generic container format used by the BRender rendering engine created by
  Argonaut Technologies. Internally, it's known as a "datafile".
  It can contain models, pixelmaps, and actors.

seq:
  - id: chunks
    type: br_datafile_chunk_t
    repeat: eos

types:
  br_datafile_chunk_t:
    seq:
      - id: type
        type: u4
      - id: len_data
        type: u4
        if: type < 65536
      - id: data
        type:
          switch-on: type
          cases:
            0: br_terminator_t # file ending
            3: br_pixelmap_t # pixelmap def
            18: br_file_index_t # file info
            22: br_material_index_t # material index
            23: br_vertex_index_t # vertices
            24: br_uv_index_t # vertex uvs
            26: br_face_material_index_t # face material assignments
            33: br_pixels_t # raw pixels
            35: br_actor_t # actor def
            36: br_actor_model_t # actor model def
            53: br_face_index_t # faces
            54: br_model_t # model def
            61: br_pixelmap_new_t # new pixelmap def
            _: br_unknown_t

  br_unknown_t:
    seq:
      - id: data
        size: _parent.len_data

  br_actor_t:
    seq:
      - id: actor_type
        type: u1
      - id: render_style
        type: u1
      - id: identifier
        type: strz
        encoding: ascii

  br_actor_model_t:
    seq:
      - id: identifier
        type: strz
        encoding: ascii

  br_terminator_t: {}

  br_file_index_t:
    seq:
      - id: type
        type: u4
      - id: version
        type: u4

  rgb_t:
    seq:
      - id: r
        type: u1
      - id: g
        type: u1
      - id: b
        type: u1

  argb_t:
    seq:
      - id: a
        type: u1
      - id: r
        type: u1
      - id: g
        type: u1
      - id: b
        type: u1

  br_pixelmap_t:
    seq:
      - id: bitmap_type
        type: b8
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

  br_pixelmap_new_t:
    seq:
      - id: bitmap_type
        type: b8
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
      - id: mip_offset
        type: u2
      - id: identifier
        type: strz
        encoding: ascii

  br_pixels_t:
    seq:
      - id: num_pixels
        type: u4
      - id: len_pixel
        type: u4
      - id: data_paletted
        type: u1
        repeat: expr
        repeat-expr: len_pixel * num_pixels
        if: len_pixel == 1
      - id: data_depth
        type: u2
        repeat: expr
        repeat-expr: (len_pixel * num_pixels) / 2
        if: len_pixel == 2
      - id: data_rgb
        type: rgb_t
        repeat: expr
        repeat-expr: (len_pixel * num_pixels) / 3
        if: len_pixel == 3
      - id: data_argb
        type: argb_t
        repeat: expr
        repeat-expr: (len_pixel * num_pixels) / 4
        if: len_pixel == 4

  br_model_t:
    seq:
      - id: flags
        type: u2
      - id: identifier
        type: strz
        encoding: ascii

  br_face_index_t:
    seq:
      - id: num_faces
        type: u4
      - id: faces
        type: br_face_t
        repeat: expr
        repeat-expr: num_faces

  br_face_t:
    seq:
      - id: vertex_indices
        type: u2
        repeat: expr
        repeat-expr: 3
      - id: smoothing
        type: u2
      - id: flags
        type: u1

  br_face_material_index_t:
    seq:
      - id: num_face_materials
        type: u4
      - id: len_face_material
        type: u4
      - id: face_materials
        type: u2
        repeat: expr
        repeat-expr: num_face_materials

  br_vertex_index_t:
    seq:
      - id: num_vertices
        type: u4
      - id: vertices
        type: br_vertex_t
        repeat: expr
        repeat-expr: num_vertices

  br_vertex_t:
    seq:
      - id: coords
        type: f4
        repeat: expr
        repeat-expr: 3

  br_uv_index_t:
    seq:
      - id: num_uvs
        type: u4
      - id: uvs
        type: br_uv_t
        repeat: expr
        repeat-expr: num_uvs

  br_uv_t:
    seq:
      - id: u
        type: f4
      - id: v
        type: f4

  br_material_index_t:
    seq:
      - id: num_materials
        type: u4
      - id: materials
        type: br_material_t
        repeat: expr
        repeat-expr: num_materials

  br_material_t:
    seq:
      - id: identifier
        type: strz
        encoding: ascii
