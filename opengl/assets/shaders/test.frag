#version 330 core
    in vec3 vertexColor;
    in vec2 texCoord;

    out vec4 FragColor;

    uniform sampler2D ourTexture;

    void main()
    {
        FragColor = texture2D(ourTexture, texCoord);
    } 