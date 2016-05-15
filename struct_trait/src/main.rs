//Struct als Komposition aus benannten Attributen. Structs sollen nach Camel-Case benant werden
struct NamedStruct {
    att_a: i32,
    att_b: i32,
    att_c: i32
}

//Struct mit anonymen Attributen, Ansprache über Position im Struct
struct AnonymStruct (i32, i8, i32);

//Definiere einen Trait um eine Funktion anzugeben
trait HasAttSum {
    fn att_sum(&self) -> i32;
}

//Definiert eine Funktion für NamedStruct wobei einem Trait genügt wird
impl HasAttSum for NamedStruct {
    fn att_sum(&self) -> i32 {
        self.att_a + self.att_b + self.att_c
    }
}

//Methode mit generischem Input welcher dem HasAttSum-Trait genügen muss
fn generic_trait_use<T: HasAttSum>(stru: T) {
    println!("Result auf att_sum method is {}", stru.att_sum());
}

fn main() {
    //Lege eine struct an
    let mut first_use = NamedStruct { att_a: 10 , att_b: 16, att_c: 20 };
    let mut second_use = AnonymStruct(40,3,7);
    println!("First struct usage has a={}, b={}, c={}", first_use.att_a, first_use.att_b, first_use.att_c);
    //Ändere 1 Attribut
    first_use.att_b = 1200;
    println!("First struct usage has a={}, b={}, c={} after change", first_use.att_a, first_use.att_b, first_use.att_c);

    println!("************");

    //Ausgabe der anonymen Struct, Ansprache über Index
    println!("Second struct usage has {}, {}, {}", second_use.0, second_use.1, second_use.2);
    second_use.2 = 350;
    println!("Second struct usage has {}, {}, {} after change", second_use.0, second_use.1, second_use.2);

    println!("************");
    generic_trait_use(first_use);
}
