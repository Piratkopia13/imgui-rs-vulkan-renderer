#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 oColor;
layout(location = 1) in vec2 oUV;

layout(binding = 0, set = 0) uniform sampler2D fontsSampler;

layout(location = 0) out vec4 finalColor;

void main() {
    finalColor = oColor * texture(fontsSampler, oUV);
    // Apply gamma correction to work with SRGB back buffer
    finalColor.r = pow(finalColor.r, 2.2f);
    finalColor.g = pow(finalColor.g, 2.2f);
    finalColor.b = pow(finalColor.b, 2.2f);
}
