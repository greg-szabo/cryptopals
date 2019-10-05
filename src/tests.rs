#[cfg(test)]
use crate::Number;
use crate::hamming;
use crate::occurrence_map;

#[test]
fn number_tests() {
    let v = vec![65, 65, 64, 32, 65, 65, 64, 32, 65, 65, 64];
    assert_eq!(
        v,
        Number::from_bytes(&vec![65, 65, 64, 32, 65, 65, 64, 32, 65, 65, 64]).to_vec()
    );
    assert_eq!(v, Number::from_str("AA@ AA@ AA@").to_vec());
    assert_eq!(v, Number::from_hexstr("4141402041414020414140").to_vec());
    assert_eq!(
        Number::from_str("AA@ AA@ AA@").to_vec(),
        Number::from_bytes(&vec![65, 65, 64, 32, 65, 65, 64, 32, 65, 65, 64]).to_vec()
    );
    assert_eq!(
        Number::from_hexstr("4141402041414020414140").to_vec(),
        Number::from_bytes(&vec![65, 65, 64, 32, 65, 65, 64, 32, 65, 65, 64]).to_vec()
    );
    assert_eq!(
        Number::from_str("AA@ AA@ AA@").to_vec(),
        Number::from_hexstr(&"4141402041414020414140").to_vec()
    );
}

#[test]
fn hamming_test() {
    assert_eq!(
        hamming(&"this is a test".as_bytes().to_vec(),&"wokka wokka!!!".as_bytes().to_vec()),
        37
    );
}

#[test]
fn occurrence_test() {
    let vector = [1,2,3,3,2,3,3,5,6,4,1].to_vec();
    assert_eq!(
        occurrence_map(&vector),
        vec![(3, 4), (1, 2), (2, 2), (4, 1), (5, 1), (6, 1)]
    )
}