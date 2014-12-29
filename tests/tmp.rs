extern crate autograd;



trait Trait {
    fn func();
}

struct Struct<'a, T: Trait> {
    x : int,
    t : T
}

#[test]
fn test2() {
    let st;
    {
        struct TStruct;
        impl Trait for TStruct {
            fn func() {
                println!("func called");
            }
        }
        st = Struct{x:0, t:TStruct};
    }
}
