use stackvec::StackVec;


#[test]
fn extend() {
    // Build a vec
    let vec: StackVec<3, String> = StackVec::from(["D".into(), "E".into(), "F".into()]);
    // Build another vec that extends
    let mut vec2: StackVec<6, String> = StackVec::from(["A".into(), "B".into(), "C".into()]);
    vec2.extend(vec);
    assert_eq!(vec2, ["A".into(), "B".into(), "C".into(), "D".into(), "E".into(), "F".into()]);
}

#[test]
fn extend_empty() {
    // Extend an empty vec
    let mut vec3: StackVec<3, String> = StackVec::from(["A".into(), "B".into(), "C".into()]);
    vec3.extend(StackVec::<3, String>::new());
    assert_eq!(vec3, ["A".into(), "B".into(), "C".into()]);
}

#[test]
fn extend_vec() {
    // Extend a vec of 1 into a vector
    let mut vec4: Vec<String> = Vec::with_capacity(5);
    vec4.extend(StackVec::<5, String>::from(["Hello there!".into(), "General Kenobi!".into()]));
    assert_eq!(vec4, vec!["Hello there!".to_string(), "General Kenobi!".to_string()]);
}

#[test]
fn extend_vec_empty() {
    // Extend an empty vec into a vector
    let mut vec4: Vec<String> = Vec::with_capacity(5);
    vec4.extend(StackVec::<5, String>::new());
    assert_eq!(vec4, Vec::<String>::new());
}

#[test]
fn retain() {
    let mut vec: StackVec<5, i32> = StackVec::from([1, 2, 3, 4, 5]);
    vec.retain(|elem| *elem >= 3);
    assert_eq!(vec, [3, 4, 5]);

    let mut vec: StackVec<5, i32> = StackVec::from([5, 4, 3, 2, 1]);
    vec.retain(|elem| *elem >= 3);
    assert_eq!(vec, [5, 4, 3]);

    let mut vec: StackVec<5, i32> = StackVec::from([4, 1, 5, 2, 3]);
    vec.retain(|elem| *elem >= 3);
    assert_eq!(vec, [4, 5, 3]);

    let mut vec: StackVec<5, String> = StackVec::from(["Dan".into(), "Amy".into(), "Cho".into(), "Eve".into(), "Bob".into()]);
    vec.retain(|elem| !elem.is_empty() && elem.chars().next().unwrap() >= 'C');
    assert_eq!(vec, ["Dan".into(), "Cho".into(), "Eve".into()]);
}

#[test]
fn retain_drain() {
    let mut vec: StackVec<5, i32> = StackVec::from([1, 2, 3, 4, 5]);
    vec.retain_drain(|elem| if elem >= 3 { Some(elem) } else { None });
    assert_eq!(vec, [3, 4, 5]);

    let mut vec: StackVec<5, i32> = StackVec::from([5, 4, 3, 2, 1]);
    vec.retain_drain(|elem| if elem >= 3 { Some(elem) } else { None });
    assert_eq!(vec, [5, 4, 3]);

    let mut vec: StackVec<5, i32> = StackVec::from([4, 1, 5, 2, 3]);
    vec.retain_drain(|elem| if elem >= 3 { Some(elem) } else { None });
    assert_eq!(vec, [4, 5, 3]);

    let mut vec: StackVec<5, String> = StackVec::from(["Dan".into(), "Amy".into(), "Cho".into(), "Eve".into(), "Bob".into()]);
    vec.retain_drain(|elem| if !elem.is_empty() && elem.chars().next().unwrap() >= 'C' { Some(elem) } else { None });
    assert_eq!(vec, ["Dan".into(), "Cho".into(), "Eve".into()]);
}

#[test]
fn sort() {
    // Do one with a from
    let mut vec: StackVec<5, &'static str> = StackVec::from(["Dan", "Amy", "Cho", "Eve", "Bob"]);
    vec.sort();
    assert_eq!(vec, ["Amy", "Bob", "Cho", "Dan", "Eve"]);
}
