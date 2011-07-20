//xfail-stage0

use std;

fn main() {

    obj a() {
        fn foo() -> int {
            ret 2;
        }
        fn bar() -> int {
            ret self.foo();
        }
    }

    auto my_a = a();

    // An anonymous object that overloads the 'foo' method.
    auto my_b = obj() {
        fn foo() -> int {
            ret 3;
        }

        with my_a
    };

    assert (my_b.foo() == 3);
    assert (my_b.bar() == 3);
}