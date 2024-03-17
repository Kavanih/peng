mod helpers;
use helpers::hash;
// Ensure this is in your lib.rs or main.rs
mod msg;


fn main() {
    let address1 = "sei1kzw6r8le78hw933q0g5xldr23wny6kcmwfwcm6";
    let address2 = "sei1xsxyrpeyrqegahmxlnply8a3dm6czxzamm5l77";

    let hashed_address1 = hash(address1);
    let hashed_address2 = hash(address2);

    println!("Hashed Address 1: {:?}", hashed_address1);
    println!("Hashed Address 2: {:?}", hashed_address2);
}
