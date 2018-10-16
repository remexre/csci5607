use cgmath::Vector3;

pub fn normal_from_points(v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
    let ab = v1 - v2;
    let bc = v2 - v3;
    ab.cross(bc)
}

pub fn normal_from_normals(
    _v1: Vector3<f32>,
    _v2: Vector3<f32>,
    _v3: Vector3<f32>,
    _n1: Vector3<f32>,
    _n2: Vector3<f32>,
    _n3: Vector3<f32>,
) -> Vector3<f32> {
    unimplemented!()
}
