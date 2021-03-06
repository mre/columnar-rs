
#[macro_use] extern crate columnar_derive;
extern crate columnar;
use columnar::Columnar;

#[derive(Eq, PartialEq, Debug, Clone, Columnar)]
pub struct Useless {
    pub a: u64,
    b: Option<i64>,
}

#[test]
fn test() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}

#[test]
fn test_mul_2() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut original = u.clone();
    for e in &mut original {
        e.a *= 2;
    }
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for e in &mut columnar {
        *e.a *= 2;
    }
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}
