struct VertexInput {
    @location(0) vert_pos: vec3<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,

};

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(model.vert_pos, 1.0);
    output.vert_pos = output.clip_position.xyz;
    return output;
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
};

struct Box {
    center: vec3<f32>,
    size: vec3<f32>,
};

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
};

const LIGHT = Light(vec3<f32>(0.0, 40.0, 5.0), vec3<f32>(0.0, 0.0, 1.0));

fn hit_spehere(ray: Ray, sphere: Sphere) -> f32 {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - sqrt(discriminant)) / (2.0 * a);
    }
}
//float sdBox( vec3 p, vec3 b )
//{
//  vec3 q = abs(p) - b;
//   return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
// }

fn sdBox(p: vec3<f32>, b: vec3<f32>) -> f32 {
    let q = abs(p) - b;
    return length(max(q, vec3(0.,0.,0.))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn sdSphere(p: vec3<f32>, s: f32) -> f32 {
    return length(p) - s;
}

fn sdf(vec: vec3<f32>) -> f32 {
    return min(sdSphere(vec-vec3<f32>(0.0,0.0,5.0), 1.0),
                sdBox(vec-vec3<f32>(0.0,-8.0,5.0), vec3<f32>(50.0,1.0,50.0)));
}

fn hit_sdf(ray: Ray) -> f32 {
    var t = 0.0;
    var p = ray.origin + t * ray.direction;
    for (var i = 0; i < 100; i = i + 1) {
        let d = sdf(p);
        if (d < 0.001) {
            return t;
        }
        t = t + d;
        p = ray.origin + t * ray.direction;
    }
    return -1.0;
}


fn hit_world(ray: Ray) -> f32 {
    let sphere = Sphere(vec3<f32>(0.0, 0.0, 5.0), 1.0);
    return hit_sdf(ray);
}


fn ray_color(ray: Ray) -> vec3<f32> {
    let unit_direction = normalize(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * vec3<f32>(1.0, 1.0, 1.0) + t * vec3<f32>(0.5, 0.7, 1.0);
}

fn trace(ray: Ray, sphere: Sphere) -> vec3<f32> {
    let t  = hit_world(ray);
    if (t> 0.0) {
        // Hit something, check light
        let hit_point = ray.origin + t * ray.direction;
        let light_direction = normalize(LIGHT.position - hit_point);
        let light_ray = Ray(hit_point, light_direction);
        let light_t = hit_world(light_ray);
        var color = vec3<f32>(0.0, 0.0, 0.0);
        if (light_t > 0.0 && light_t < length(LIGHT.position - hit_point)) {
            // Light is blocked
            color += vec3<f32>(0.0, 0.0, 0.0);
        } else {
            // Light is not blocked
            color += dot(light_direction, normalize(hit_point - sphere.center)) * LIGHT.color;
        }
        return color;
    } else {
        return ray_color(ray);
    }
}

fn primary_ray(x: f32, y: f32) -> Ray {
    let origin = vec3<f32>(0.0, 0.0, 0.0);
    let direction = normalize(vec3<f32>(x, y, 1.0));
    return Ray(origin, direction);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sphere = Sphere(vec3<f32>(0.0, 0.0, 5.0), 1.0);
    let ray = primary_ray(in.vert_pos.x, in.vert_pos.y);
    let cor = trace(ray, sphere);
    return vec4<f32>(cor, 1.0);
}