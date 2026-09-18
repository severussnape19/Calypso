struct VSInput {
    [[vk::location(0)]] float3 inPosition;
    [[vk::location(1)]] float3 inColor;
    [[vk::location(2)]] float2 inUV;
    [[vk::location(3)]] float3 inNormal;
};

struct UniformBuffer {
    float4x4 model;
    float4x4 view;
    float4x4 proj;
};
ConstantBuffer<UniformBuffer> ubo;

struct VSOutput {
    float4 pos: SV_Position;
    float3 frag_color;
    float2 frag_uv;
    float3 frag_normal;
};

Sampler2D texture;

[shader("vertex")]
VSOutput vertMain(VSInput input) {
    VSOutput output;
    float4 position = float4(input.inPosition, 1.0);
    output.pos = mul(ubo.proj, mul(ubo.view, mul(ubo.model, position))); // MVP
    output.frag_color = input.inColor;
    output.frag_uv = input.inUV;
    output.frag_normal = input.inNormal;
    return output;
}

[shader("fragment")]
float4 fragMain(VSOutput inVert) : SV_Target {
    float3 n = normalize(inVert.frag_normal);
    float3 color = n * 0.5 + 0.5;
    return float4(color, 1.0);
}
