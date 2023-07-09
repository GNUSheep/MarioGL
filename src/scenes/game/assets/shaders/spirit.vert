#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 posTex;

uniform vec2 movePos;
uniform int flipTex;

uniform mat4 view;

out vec2 texPos;

void main()
{
    gl_Position = vec4(pos.x+movePos.x, pos.y+movePos.y, pos.z, 1.0) * view;
    if (flipTex == 1) {
        texPos = vec2(1.0-posTex.x, posTex.y);
    }else {
        texPos = vec2(posTex.x, posTex.y);
    }
}    