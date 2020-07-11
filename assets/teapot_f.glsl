#version 330 core
out vec4 FragColor;

in vec3 Normal;  
in vec3 FragPos;

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    uint gloss;
};

struct Light {
    vec3 color;
    vec4 vector;
    float range;
};

uniform Light light;
uniform vec3 viewPos;
uniform Material material;

void main()
{
    //if (light.vector.w == 1.0) {
        vec3  lightDir    = normalize(vec3(light.vector.x, light.vector.y, light.vector.z) - FragPos);
        float distance    = length(vec3(light.vector.x, light.vector.y, light.vector.z) - FragPos);
        float attenuation = 1.0 / pow(distance, 2);
    //}

    vec3 norm = normalize(Normal);

    // ambient
    vec3 ambient = light.color * material.ambient;

    // diffuse
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.color * (diff * material.diffuse);
    diffuse *= attenuation;

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.gloss);
    vec3 specular = light.color * (spec * material.gloss);
    specular *= attenuation;

    vec3 result = ambient + (diffuse + specular);
    FragColor = vec4(result, 1.0);
}

