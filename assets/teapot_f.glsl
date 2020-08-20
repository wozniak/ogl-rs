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
    vec4 position;
    vec3 direction;
    float flashlightCutoff;
    float range;
    float linear;
    float quadratic;
    float radius;
};

uniform Light light;
uniform vec3 viewPos;
uniform Material material;

void main()
{
    // point light


    /*vec3  lightDir    = normalize(vec3(light.position.x, light.position.y, light.position.z) - FragPos);
    float distance    = length(vec3(light.position.x, light.vector.y, light.vector.z) - FragPos);
    float attenuation = 1.0 / (1.0 + light.linear * distance +
        light.quadratic * (distance * distance));
    */

    // sun light


    vec3 lightDir = -normalize(vec3(light.direction.x, light.direction.y, light.direction.z));
    float attenuation = 1.0;


    // spot light
    /*
    float theta = dot(lightDir, normalize(-light.direction));

    if(theta > light.flashlightCutoff)
    {
      float attenuation = 1.0 / (1.0 + light.linear * distance +
        light.quadratic * (distance * distance));
    }
    else  // else, use ambient light so scene isn't completely dark outside the spotlight.
      float attenuation = 0.1;
    */


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

