#version 150

uniform AdditionalData {
    vec2 center;
    vec2 size;
    vec2 resolution;
    int iter;
};

out vec4 Target0;

void main() {
    vec2 z, c;

    c.x = center.x - size.x/2.0 + gl_FragCoord.x * size.x / resolution.x;
    c.y = center.y - size.y/2.0 + gl_FragCoord.y * size.y / resolution.y;

    int i;
    z = c;
    for(i=0; i<iter; i++) {
        float x = (z.x * z.x - z.y * z.y) + c.x;
        float y = (z.y * z.x + z.x * z.y) + c.y;

        if((x * x + y * y) > 4.0) break;
        z.x = x;
        z.y = y;
    }

    float brightness = sin(float(i)/float(iter)*3.141592653);

    Target0 = vec4(vec3(brightness), 1.0);
}
