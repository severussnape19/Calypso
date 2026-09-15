struct VSInput {
    [[vk::location(0)]] float3 inPosition;
    [[vk::location(1)]] float3 inColor;
    [[vk::location(2)]] float2 inUV;
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
};

Sampler2D texture;

[shader("vertex")]
VSOutput vertMain(VSInput input) {
    VSOutput output;
    output.pos = mul(ubo.proj, mul(ubo.view, mul(ubo.model, float4(input.inPosition, 1.0))));
    output.frag_color = input.inColor;
    output.frag_uv = input.inUV;
    return output;
}

[shader("fragment")]
float4 fragMain(VSOutput inVert) : SV_Target {
    float3 color = inVert.frag_color;
    return float4(texture.Sample(inVert.frag_uv).rgb, 1.0);
}
