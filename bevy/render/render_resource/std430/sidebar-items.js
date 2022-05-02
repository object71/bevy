initSidebarItems({"derive":[["AsStd430",""]],"enum":[["InvalidPadded","Unfortunately, we cannot easily derive padded representation for generic Std140 types. For now, we’ll just use this empty enum with no values."]],"struct":[["DMat2","Corresponds to a GLSL `dmat2` in std430 layout."],["DMat3","Corresponds to a GLSL `dmat3` in std430 layout."],["DMat4","Corresponds to a GLSL `dmat3` in std430 layout."],["DVec2","Corresponds to a GLSL `dvec2` in std430 layout."],["DVec3","Corresponds to a GLSL `dvec3` in std430 layout."],["DVec4","Corresponds to a GLSL `dvec4` in std430 layout."],["IVec2","Corresponds to a GLSL `ivec2` in std430 layout."],["IVec3","Corresponds to a GLSL `ivec3` in std430 layout."],["IVec4","Corresponds to a GLSL `ivec4` in std430 layout."],["Mat2","Corresponds to a GLSL `mat2` in std430 layout."],["Mat3","Corresponds to a GLSL `mat3` in std430 layout."],["Mat4","Corresponds to a GLSL `mat4` in std430 layout."],["Sizer","Type that computes the buffer size needed by a series of `std430` types laid out."],["UVec2","Corresponds to a GLSL `uvec2` in std430 layout."],["UVec3","Corresponds to a GLSL `uvec3` in std430 layout."],["UVec4","Corresponds to a GLSL `uvec4` in std430 layout."],["Vec2","Corresponds to a GLSL `vec2` in std430 layout."],["Vec3","Corresponds to a GLSL `vec3` in std430 layout."],["Vec4","Corresponds to a GLSL `vec4` in std430 layout."],["Writer","Type that enables writing correctly aligned `std430` values to a buffer."]],"trait":[["AsStd430","Trait implemented for all types that can be turned into `std430` values."],["Std430","Trait implemented for all `std430` primitives. Generally should not be implemented outside this crate."],["Std430Convertible","Trait specifically for Std430::Padded, implements conversions between padded type and base type."],["WriteStd430","Trait implemented for all types that can be written into a buffer as `std430` bytes. This type is more general than [`AsStd430`]: all `AsStd430` types implement `WriteStd430`, but not the other way around."]]});