//Methode mit generischem Input welcher dem HasAttSum-Trait genügen muss
fn generic_trait_use<T: HasAttSum>(stru: T) {
    println!("Result auf att_sum method is {}", stru.att_sum());
}
