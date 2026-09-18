use glm::{ Vec2, Vec3 };

/* vao, ibo and Vertex */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos:    Vec3,
    pub color:  Vec3,
    pub uv:     Vec2, // texture co-ordinates
    pub normal: Vec3
}

impl Vertex {
    pub fn get_binding_description() -> ash::vk::VertexInputBindingDescription {
        ash::vk::VertexInputBindingDescription {
            binding: 0u32,
            stride: size_of::<Vertex>() as u32,
            input_rate: ash::vk::VertexInputRate::VERTEX,
        }
    }

    pub fn get_attribute_descriptions() -> [ash::vk::VertexInputAttributeDescription; 4] {
        let position_description = ash::vk::VertexInputAttributeDescription::default()
            .location(0u32)
            .binding(0u32)
            .format(ash::vk::Format::R32G32B32_SFLOAT)
            .offset(std::mem::offset_of!(Vertex, pos) as u32);

        let color_description = ash::vk::VertexInputAttributeDescription::default()
            .location(1u32)
            .binding(0u32)
            .format(ash::vk::Format::R32G32B32_SFLOAT)
            .offset(std::mem::offset_of!(Vertex, color) as u32);

        let uv_description = ash::vk::VertexInputAttributeDescription::default()
            .location(2_u32)
            .binding(0_u32)
            .format(ash::vk::Format::R32G32_SFLOAT)
            .offset(std::mem::offset_of!(Vertex, uv) as u32);

        let normal_description = ash::vk::VertexInputAttributeDescription::default()
            .location(3_u32)
            .binding(0_u32)
            .format(ash::vk::Format::R32G32B32_SFLOAT)
            .offset(std::mem::offset_of!(Vertex, normal) as u32);

        [position_description, color_description, uv_description, normal_description]
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices:  Vec<u16>
}

impl Mesh {
   pub fn data() -> Self {
    Self {
        vertices: vec![
            // Front face (+Z) - Normal: (0.0, 0.0, 1.0)
            Vertex { pos: Vec3::new(-0.5, -0.5,  0.5), color: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, 0.0, 1.0) },
            Vertex { pos: Vec3::new( 0.5, -0.5,  0.5), color: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(1.0, 0.0), normal: Vec3::new(0.0, 0.0, 1.0) },
            Vertex { pos: Vec3::new( 0.5,  0.5,  0.5), color: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(1.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0) },
            Vertex { pos: Vec3::new(-0.5,  0.5,  0.5), color: Vec3::new(1.0, 1.0, 0.0), uv: Vec2::new(0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0) },

            // Back face (-Z) - Normal: (0.0, 0.0, -1.0)
            Vertex { pos: Vec3::new(-0.5, -0.5, -0.5), color: Vec3::new(1.0, 0.0, 1.0), uv: Vec2::new(1.0, 0.0), normal: Vec3::new(0.0, 0.0, -1.0) },
            Vertex { pos: Vec3::new( 0.5, -0.5, -0.5), color: Vec3::new(0.0, 1.0, 1.0), uv: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, 0.0, -1.0) },
            Vertex { pos: Vec3::new( 0.5,  0.5, -0.5), color: Vec3::new(1.0, 1.0, 1.0), uv: Vec2::new(0.0, 1.0), normal: Vec3::new(0.0, 0.0, -1.0) },
            Vertex { pos: Vec3::new(-0.5,  0.5, -0.5), color: Vec3::new(0.2, 0.2, 0.2), uv: Vec2::new(1.0, 1.0), normal: Vec3::new(0.0, 0.0, -1.0) },

            // Right face (+X) - Normal: (1.0, 0.0, 0.0)
            Vertex { pos: Vec3::new( 0.5, -0.5,  0.5), color: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(0.0, 0.0), normal: Vec3::new(1.0, 0.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5, -0.5, -0.5), color: Vec3::new(0.0, 1.0, 1.0), uv: Vec2::new(1.0, 0.0), normal: Vec3::new(1.0, 0.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5,  0.5, -0.5), color: Vec3::new(1.0, 1.0, 1.0), uv: Vec2::new(1.0, 1.0), normal: Vec3::new(1.0, 0.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5,  0.5,  0.5), color: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 1.0), normal: Vec3::new(1.0, 0.0, 0.0) },

            // Left face (-X) - Normal: (-1.0, 0.0, 0.0)
            Vertex { pos: Vec3::new(-0.5, -0.5, -0.5), color: Vec3::new(1.0, 0.0, 1.0), uv: Vec2::new(0.0, 0.0), normal: Vec3::new(-1.0, 0.0, 0.0) },
            Vertex { pos: Vec3::new(-0.5, -0.5,  0.5), color: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(1.0, 0.0), normal: Vec3::new(-1.0, 0.0, 0.0) },
            Vertex { pos: Vec3::new(-0.5,  0.5,  0.5), color: Vec3::new(1.0, 1.0, 0.0), uv: Vec2::new(1.0, 1.0), normal: Vec3::new(-1.0, 0.0, 0.0) },
            Vertex { pos: Vec3::new(-0.5,  0.5, -0.5), color: Vec3::new(0.2, 0.2, 0.2), uv: Vec2::new(0.0, 1.0), normal: Vec3::new(-1.0, 0.0, 0.0) },

            // Top face (+Y) - Normal: (0.0, 1.0, 0.0)
            Vertex { pos: Vec3::new(-0.5,  0.5,  0.5), color: Vec3::new(1.0, 1.0, 0.0), uv: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5,  0.5,  0.5), color: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(1.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5,  0.5, -0.5), color: Vec3::new(1.0, 1.0, 1.0), uv: Vec2::new(1.0, 1.0), normal: Vec3::new(0.0, 1.0, 0.0) },
            Vertex { pos: Vec3::new(-0.5,  0.5, -0.5), color: Vec3::new(0.2, 0.2, 0.2), uv: Vec2::new(0.0, 1.0), normal: Vec3::new(0.0, 1.0, 0.0) },

            // Bottom face (-Y) - Normal: (0.0, -1.0, 0.0)
            Vertex { pos: Vec3::new(-0.5, -0.5, -0.5), color: Vec3::new(1.0, 0.0, 1.0), uv: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, -1.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5, -0.5, -0.5), color: Vec3::new(0.0, 1.0, 1.0), uv: Vec2::new(1.0, 0.0), normal: Vec3::new(0.0, -1.0, 0.0) },
            Vertex { pos: Vec3::new( 0.5, -0.5,  0.5), color: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(1.0, 1.0), normal: Vec3::new(0.0, -1.0, 0.0) },
            Vertex { pos: Vec3::new(-0.5, -0.5,  0.5), color: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(0.0, 1.0), normal: Vec3::new(0.0, -1.0, 0.0) },
        ],
        indices: vec![
            0,  1,  2,   2,  3,  0, // +Z face
            5,  4,  7,   7,  6,  5, // -Z face
            8,  9, 10,  10, 11,  8, // +X face
           12, 13, 14,  14, 15, 12, // -X face
           16, 17, 18,  18, 19, 16, // +Y face
           20, 21, 22,  22, 23, 20, // -Y face
        ],
    }
}
}
