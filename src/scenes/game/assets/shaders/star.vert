#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 posTex;

uniform vec2 movePos;
uniform mat4 view;

out vec2 texPos;

void main()
{
    gl_Position = vec4(pos.x+movePos.x, pos.y+movePos.y, pos.z, 1.0) * view ;
    texPos = posTex;
}    