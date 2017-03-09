extern crate parol;

use parol::core::{Parol, Parols};

#[test]
fn test_json() {
    let mut parols = Parols::new();

    for i in 0 .. 10 {
        let parol = Parol::new_with_arguments("tox", "Ogromny", "superstrongpassword", "");
        parols.push(parol);
    }

    let parols2 = Parols::new_from_json(&parols.to_json());

    // dirty
    for i in 0 .. 10 {
        let parols_i = match parols.get(i) {
            Some(parols) => parols,
            None => panic!("..."),
        };
        let parols2_i = match parols2.get(i) {
            Some(parols) => parols,
            None => panic!("..."),
        };

        assert_eq!(parols_i.get_application(), parols2_i.get_application());
        assert_eq!(parols_i.get_username(), parols2_i.get_username());
        assert_eq!(parols_i.get_password(), parols2_i.get_password());
        assert_eq!(parols_i.get_notes(), parols2_i.get_notes());
    }

    assert_eq!(parols.len(), parols2.len());
}