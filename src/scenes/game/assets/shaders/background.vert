#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 posTex;

out vec2 texPos;

void main()
{
    gl_Position = vec4(pos, 1.0);
    texPos = posTex;
}    