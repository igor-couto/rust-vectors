use super::*;

#[test]
fn should_create_origin() {
    let origin = Vector::origin();
    let origin2 = Vector { x: 0f32, y: 0f32 };
    assert!(origin == origin2)
}

// Add

#[test]
fn should_add_with_operator() {
    let v1 = Vector::new(2f32, 5f32);
    let v2 = Vector::new(3f32, 4f32);

    let result = v1 + v2;

    assert!(result.x == 5f32 && result.y == 9f32);
}

#[test]
fn should_add_values_to_itself() {
    let mut v1 = Vector::new(2f32, 5f32);

    v1.add_values(3f32, 4f32);

    assert!(v1.x == 5f32 && v1.y == 9f32);
}

#[test]
fn should_add_vector_to_itself() {
    let mut v1 = Vector::new(2f32, 5f32);
    let v2 = Vector::new(3f32, 4f32);

    v1.add_vector(&v2);

    assert!(v1.x == 5f32 && v1.y == 9f32);
}

// Subtract

#[test]
fn should_subtract_with_operator() {
    let v1 = Vector::new(2f32, 5f32);
    let v2 = Vector::new(3f32, 4f32);

    let result = v1 - v2;

    assert!(result.x == -1f32 && result.y == 1f32);
}

#[test]
fn should_subtract_values_to_itself() {
    let mut v1 = Vector::new(2f32, 5f32);

    v1.sub_values(3f32, 4f32);

    assert!(v1.x == -1f32 && v1.y == 1f32);
}

#[test]
fn should_subtract_vector_to_itself() {
    let mut v1 = Vector::new(2f32, 5f32);
    let v2 = Vector::new(3f32, 4f32);

    v1.sub_vector(&v2);

    assert!(v1.x == -1f32 && v1.y == 1f32);
}

// Other

#[test]
fn should_calculate_magnitude() {
    let vector = Vector::new(4f32, 3f32);

    assert_eq!(vector.mag(), 5f32);
}

#[test]
fn should_normalize() {
    let mut vector = Vector::new(4f32, 3f32);

    vector.normalize();

    assert!(vector.mag() == 1.0 && vector.x == 4.0 / 5.0 && vector.y == 3.0 / 5.0);
}
