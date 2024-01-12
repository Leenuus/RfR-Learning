#![allow(dead_code)]

pub mod memory {
    pub fn move_by_mutable_reference() {
        #[derive(Default)]
        struct A;

        fn move_by_mutable_reference_inner(b: &mut Box<A>) {
            // not compile, move the value in function body
            // and the value is dropped after this function
            // conflicts with the &mut which is not intended to move the value
            // let was = *b;

            // mem::take usage
            // it only asks for &mut
            // b should implement `Default` trait
            // value of b after being taken is
            // A::default()
            std::mem::take(b);
        }

    }

    pub fn borrow_checker_work_bottom_up() {
        let mut x = Box::new(1);
        let r = &x; // 1
        if true {
            println!("{r}"); // 2
        } else {
            // the same if you modify the origin value by variable name but not mutable reference
            *x = 84; // 3
            // not work too
            // we can __not__ mutate the value of x after it is borrowed immutablly
            // unless the reference borrowed has been used/consumed
            // x = Box::new(4);
        }
        // 4
        // won't work
        // *x ask for mutable reference to change the value
        // and Rust compiler found out follow this branch
        // a mutable reference is needed between immutable reference `r`'s creation and use
        // It can never gurantee the immutable reference `r` is the same value as before
        // which is what semantic immutable reference has
        // println!("{r}");
    }

    pub fn two_lifetime_annotations() {
        struct StrSplit<'s, 'p> {
            delimiter: &'p str,
            // should be __different__ lifetime identifier from p
            document: &'s str,
        }
        impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
            type Item = &'s str;
            fn next(&mut self) -> Option<Self::Item> {
                todo!()
            }
        }
        fn str_before(s: &str, c: char) -> Option<&str> {
            // if StrSplit is defined with only one lifetime identifier
            // otherwise, compiler asks us to provide a delimiter which
            // has the same lifetime as s
            // `&c.to_string()` in this function has a lifetime constrained to this function
            // it can't be as same as `s`
            // so it won't compile without different lifetime annotations
            StrSplit {
                document: s,
                delimiter: &c.to_string(),
            }
            .next()
        }
    }

    pub fn crazy_lifetime_variance() {
        let mut s = "hello";
        struct MutStr<'a, 'b> {
            s: &'a mut &'b str,
        }
        // don't work
        // struct MutStr<'a> {
        //     s: &'a mut &'a str,
        // }
        // change `s` by dereference the mutable reference
        // inside immediate drop struct `MutStr`'s `s` field
        *MutStr { s: &mut s }.s = "world"; 
        println!("{}", s);
    }
}

pub mod layout {
    pub fn memory_layout() {
        #[repr(C)]
        struct Clayout {
            f1: u8,
            f2: u32,
            f3: u8,
            f4: u64,
        }
        // debug build
        // should be f1, padding 3 bytes, f2, f3, padding 7 bytes, f4
        let c = Clayout {
            f1: 1,
            f2: 1,
            f3: 1,
            f4: 1,
        };
        // f1: 0x7fff9d4ead80, f2: 0x7fff9d4ead84, f3: 0x7fff9d4ead88, f4: 0x7fff9d4ead90
        println!(
            "f1: {:p}, f2: {:p}, f3: {:p}, f4: {:p}",
            &c.f1, &c.f2, &c.f3, &c.f4
        );
        struct Rlayout {
            f1: u8,
            f2: u32,
            f3: u8,
            f4: u64,
        }
        // debug build
        // should be f4, f2, f1, f3
        let r = Rlayout {
            f1: 1,
            f2: 1,
            f3: 1,
            f4: 1,
        };
        println!(
            "f1: {:p}, f2: {:p}, f3: {:p}, f4: {:p}",
            &r.f1, &r.f2, &r.f3, &r.f4
        );
        // f1: 0x7fff9d4eae34, f2: 0x7fff9d4eae30, f3: 0x7fff9d4eae35, f4: 0x7fff9d4eae28
        #[repr(packed)]
        struct Playout {
            f1: u8,
            f2: u32,
            f3: u8,
            f4: u64,
        }
        let _p = Playout {
            f1: 1,
            f2: 1,
            f3: 1,
            f4: 1,
        };
        // not work, because this type is not aligned
        // some of these addresses can not be printed(not byte aligned)
        // println!(
        //     "f1: {:p}, f2: {:p}, f3: {:p}, f4: {:p}",
        //     &_p.f1, &_p.f2, &_p.f3, &_p.f4
        // );
        // TODO transparent layout
    }

    pub fn array_layout() {
        // NOTE array elements have no padding in between
        // but the padding of struct still exists
        #[derive(Debug, Default)]
        struct Byte9 {
            _f1: u8,
            _f2: u64,
        }
        let a = Byte9::default();
        let b = Byte9::default();
        let c = Byte9::default();
        let arr = [a, b, c];
        let mut i = 0;
        loop {
            if i == arr.len(){
                break;
            }
            println!("element {i}: {:p}", &arr[i]);
            i += 1;
        }
        // element 0: 0x7fff2c315398
        // element 1: 0x7fff2c3153a8
        // element 2: 0x7fff2c3153b8
    }

    pub fn enumeration_layout() {
        // TODO inspect enumeration memory layout in Rust
        // seems hard to do it in `GDB`
        #[derive(Debug)]
        enum A {
            V1(u8),
            V2(u64),
        }
        let _a = A::V1(1);
        let _b = A::V2(1);
    }
}
