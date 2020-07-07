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
    float strength;
    vec4 position;
};

uniform Light light;
uniform vec3 viewPos;
uniform Material material;

void main()
{
    if (light.position.w == 0) {
        lightDir = vec3(light.position.x, light.position.y, light.position.z);
    } else {
        lightDir = normalize(vec3(vec3(light.position.x, light.position.y, light.position.z) - FragPos));
    }

    // vec3 lightDir = normalize(light.position - FragPos);

    vec3 norm = normalize(Normal);

    // ambient
    vec3 ambient = light.color * material.ambient;

    // diffuse
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.color * (diff * material.diffuse);

    // specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.gloss);
    vec3 specular = light.color * (spec * material.gloss);

    vec3 result = ambient + (diffuse + specular) * light.strength;
    FragColor = vec4(result, 1.0);
}

