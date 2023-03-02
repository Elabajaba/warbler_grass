#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct ShaderRegionConfiguration {
    main_color: vec4<f32>,
    bottom_color: vec4<f32>,
    wind: vec2<f32>,
    _wasm_padding: vec2<f32>,
};
@group(1) @binding(0)
var<uniform> mesh: Mesh;

@group(2) @binding(0)
var<uniform> config: ShaderRegionConfiguration;

@group(2) @binding(1)
var noise_texture: texture_2d<f32>;

#ifdef HEIGHT_MAP
    @group(3) @binding(0)
    var height_map: texture_2d<f32>;

    @group(3) @binding(1)
    var<uniform> aabb: vec3<f32>;
#else
    @group(3) @binding(0)
    var y_positions: texture_2d<f32>;
    
#endif
@group(4) @binding(0)
var xz_positions: texture_2d<f32>;
@group(5) @binding(0)
var heights: texture_2d<f32>;
#import bevy_pbr::mesh_functions

struct Vertex {
    // position of the local vertex in the blade
    @location(0) position: vec3<f32>,
    // // xz position of the blade as an instance
    // @location(1) xz_offset: vec2<f32>,
    // // height of the blade
    // @location(2) height: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

const NOISE_TEXTURE_SPEED: f32 = 30.;
const NOISE_TEXTURE_ZOOM: f32 = 5.;

fn wind_offset(vertex_position: vec2<f32>) -> vec2<f32> {
    var texture_offset = config.wind.xy * globals.time * NOISE_TEXTURE_SPEED;
    var texture_position = vec2<f32>(vertex_position.x ,vertex_position.y) * NOISE_TEXTURE_ZOOM + texture_offset;
    
    // dimensions of noise texture in vec2<u32>
    let dim = textureDimensions(noise_texture, 0);

    // read just position in case of a over/under flow of tex. coords
    texture_position = abs(texture_position % vec2<f32>(dim));
    var texture_pixel = textureLoad(noise_texture, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0);
    return texture_pixel.xy * config.wind;
}
#ifdef HEIGHT_MAP
    fn height_map_offset(vertex_position: vec2<f32>) -> f32 {
        let dim = textureDimensions(height_map, 0);
        let texture_position = abs((vertex_position.xy / aabb.xz ) * vec2<f32>(dim)) ;
        var texture_r = textureLoad(height_map, vec2<i32>(i32(texture_position.x),i32(texture_position.y)), 0).r;
        return texture_r * aabb.y;
    }
#endif
fn get_pixel(index: u32, texture: texture_2d<f32>) -> vec4<f32> {
    let dim = vec2<u32>(textureDimensions(texture, 0));
    let coord = vec2<u32>(index % dim.x, index / dim.x);
    let pixel = textureLoad(texture,coord,0);
    return(pixel);
}
@vertex
fn vertex(vertex: Vertex, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // load xz positions
    let xz_pixel = get_pixel(instance_index, xz_positions);
    var position_field_offset = vec3<f32>(xz_pixel.r, 0.,xz_pixel.g);

    // position of the vertex in the y_texture
    #ifdef HEIGHT_MAP
        position_field_offset.y = height_map_offset(vertex.xz_offset);
    #else
        let dim = vec2<u32>(textureDimensions(y_positions, 0));
        let y_coord = vec2<u32>(instance_index % dim.x, instance_index / dim.x);
        position_field_offset.y = textureLoad(y_positions, y_coord, 0).r;
    #endif
    let height = get_pixel(instance_index, heights).r;
    var position = vertex.position * vec3<f32>(1.,height, 1.) + position_field_offset;

    // only applies wind if the vertex is not on the bottom of the grass (or very small)
    let offset = wind_offset(position_field_offset.xz);
    let strength = max(0.,log(vertex.position.y + 1.));
    position.x += offset.x * strength;
    position.z += offset.y * strength;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));

    let lambda = clamp(vertex.position.y, 0.,1.);
    out.color = mix(config.bottom_color, config.main_color, lambda);
    return out;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}