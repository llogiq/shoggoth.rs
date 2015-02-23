#![feature(plugin)]
#![feature(test)]
#![plugin(shoggoth_plugins)]

extern crate shoggoth;
extern crate test;

#[bench]
fn bench(b: &mut test::Bencher) {
    let task = || {
        let _: Nat!(2048) = nat!(1024) + nat!(1024);
    };
    b.iter(task);
}
