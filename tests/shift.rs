use ibig::prelude::*;

#[test]
fn test_ubig_shl() {
    assert_eq!(ubig!(0) << 17u32, ubig!(0));
    assert_eq!(&ubig!(0) << 17u32, ubig!(0));
    assert_eq!(ubig!(0) << (1u128 << 127), ubig!(0));
    assert_eq!(&ubig!(0) << (1u128 << 127), ubig!(0));
    assert_eq!(ubig!(0) << (ubig!(1) << 1000u32), ubig!(0));

    assert_eq!(ubig!(0xef) << 0u32, ubig!(0xef));
    assert_eq!(ubig!(0xef) << 4u32, ubig!(0xef0));
    assert_eq!(ubig!(0xef) << ubig!(4), ubig!(0xef0));
    assert_eq!(
        ubig!(0xef) << 128u32,
        ubig!(_0xef00000000000000000000000000000000)
    );
    assert_eq!(
        ubig!(0xef) << 124u32,
        ubig!(_0xef0000000000000000000000000000000)
    );
    assert_eq!(ubig!(0xef) << 10240u32, ubig!(_0xef0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
    assert_eq!(ubig!(0xef) << 10244u32, ubig!(_0xef00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));

    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef) << 0u32,
        ubig!(_0x0123456789abcdef0123456789abcdef)
    );
    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef) << 4u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0)
    );
    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef) << 128u32,
        ubig!(_0x0123456789abcdef0123456789abcdef00000000000000000000000000000000)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef) << 128u32,
        ubig!(_0x0123456789abcdef0123456789abcdef00000000000000000000000000000000)
    );
    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef) << 124u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0000000000000000000000000000000)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef) << 124u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0000000000000000000000000000000)
    );
    assert_eq!(ubig!(_0x0123456789abcdef0123456789abcdef) << 10240u32,
               ubig!(_0x0123456789abcdef0123456789abcdef0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));
    assert_eq!(ubig!(_0x0123456789abcdef0123456789abcdef) << 10244u32,
               ubig!(_0x0123456789abcdef0123456789abcdef00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000));

    assert_eq!(&ubig!(0xef) << 4u32, ubig!(0xef0));
    assert_eq!(ubig!(0xef) << &4u32, ubig!(0xef0));
    assert_eq!(&ubig!(0xef) << &4u32, ubig!(0xef0));

    assert_eq!(&ubig!(0xef) << ubig!(4), ubig!(0xef0));
    assert_eq!(ubig!(0xef) << &ubig!(4), ubig!(0xef0));
    assert_eq!(&ubig!(0xef) << &ubig!(4), ubig!(0xef0));

    assert_eq!(ubig!(0xef) << 4i32, ubig!(0xef0));
    assert_eq!(&ubig!(0xef) << 4i32, ubig!(0xef0));
    assert_eq!(ubig!(0xef) << &4i32, ubig!(0xef0));
    assert_eq!(&ubig!(0xef) << &4i32, ubig!(0xef0));

    assert_eq!(ubig!(0xef) << ibig!(4), ubig!(0xef0));
    assert_eq!(&ubig!(0xef) << ibig!(4), ubig!(0xef0));
    assert_eq!(ubig!(0xef) << &ibig!(4), ubig!(0xef0));
    assert_eq!(&ubig!(0xef) << &ibig!(4), ubig!(0xef0));
}

#[test]
#[should_panic]
fn test_ubig_shl_too_large() {
    let _ = ubig!(1) << (1u128 << 127);
}

#[test]
#[should_panic]
fn test_ubig_shl_negative() {
    let _ = ubig!(1) << (-3i32);
}

#[test]
fn test_ibig_shl() {
    assert_eq!(ibig!(0) << 4u32, ibig!(0));
    assert_eq!(ibig!(0) << (1u128 << 127), ibig!(0));

    assert_eq!(ibig!(0xef) << 4u32, ibig!(0xef0));
    assert_eq!(&ibig!(0xef) << 4u32, ibig!(0xef0));
    assert_eq!(ibig!(0xef) << &4u32, ibig!(0xef0));
    assert_eq!(&ibig!(0xef) << &4u32, ibig!(0xef0));

    assert_eq!(ibig!(0xef) << ubig!(4), ibig!(0xef0));
    assert_eq!(&ibig!(0xef) << ubig!(4), ibig!(0xef0));
    assert_eq!(ibig!(0xef) << &ubig!(4), ibig!(0xef0));
    assert_eq!(&ibig!(0xef) << &ubig!(4), ibig!(0xef0));

    assert_eq!(ibig!(0xef) << ibig!(4), ibig!(0xef0));
    assert_eq!(&ibig!(0xef) << ibig!(4), ibig!(0xef0));
    assert_eq!(ibig!(0xef) << &ibig!(4), ibig!(0xef0));
    assert_eq!(&ibig!(0xef) << &ibig!(4), ibig!(0xef0));
}

#[test]
fn test_shl_assign() {
    let mut a = ubig!(0xef);
    a <<= 4;
    assert_eq!(a, ubig!(0xef0));
    a <<= &4;
    assert_eq!(a, ubig!(0xef00));
}

#[test]
fn test_ubig_shr() {
    assert_eq!(ubig!(0xef) >> 4u32, ubig!(0xe));
    assert_eq!(ubig!(0xef) >> 4u128, ubig!(0xe));
    assert_eq!(ubig!(0xef) >> ubig!(4), ubig!(0xe));

    assert_eq!(ubig!(0xef) >> 100, ubig!(0));
    assert_eq!(&ubig!(0xef) >> 100, ubig!(0));
    assert_eq!(ubig!(0xef) >> (1u128 << 127), ubig!(0));
    assert_eq!(&ubig!(0xef) >> (1u128 << 127), ubig!(0));
    assert_eq!(ubig!(0xef) >> (ubig!(1) << 1000), ubig!(0));

    assert_eq!((ubig!(0xef) << 63u32) >> 63u32, ubig!(0xef));
    assert_eq!((ubig!(0xef) << 64u32) >> 64u32, ubig!(0xef));
    assert_eq!((ubig!(0xef) << 1023u32) >> 1023u32, ubig!(0xef));
    assert_eq!((ubig!(0xef) << 1024u32) >> 1024u32, ubig!(0xef));
    assert_eq!(((ubig!(0xef) << 1024u32) >> 512u32) >> 512u32, ubig!(0xef));
    assert_eq!(((ubig!(0xef) << 1024u32) >> 510u32) >> 514u32, ubig!(0xef));
    assert_eq!(
        &(&(ubig!(0xef) << 1024u32) >> 512u32) >> 512u32,
        ubig!(0xef)
    );
    assert_eq!(
        &(&(ubig!(0xef) << 1024u32) >> 510u32) >> 514u32,
        ubig!(0xef)
    );

    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 0u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef)
    );

    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 0u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef)
    );

    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 1000000u32,
        ubig!(0)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 1000000u32,
        ubig!(0)
    );
    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 1000001u32,
        ubig!(0)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 1000001u32,
        ubig!(0)
    );

    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 4u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcde)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 4u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcde)
    );

    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 64u32,
        ubig!(_0x0123456789abcdef0123456789abcdef)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 64u32,
        ubig!(_0x0123456789abcdef0123456789abcdef)
    );
    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 124u32,
        ubig!(_0x123456789abcdef0)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 124u32,
        ubig!(_0x123456789abcdef0)
    );
    assert_eq!(
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 128u32,
        ubig!(_0x123456789abcdef)
    );
    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 128u32,
        ubig!(_0x123456789abcdef)
    );

    assert_eq!(ubig!(0xef) >> &4u32, ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> 4u32, ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> &4u32, ubig!(0xe));

    assert_eq!(ubig!(0xef) >> &ubig!(4), ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> ubig!(4), ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> &ubig!(4), ubig!(0xe));

    assert_eq!(
        &ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcdef) >> 4u32,
        ubig!(_0x0123456789abcdef0123456789abcdef0123456789abcde)
    );

    assert_eq!(ubig!(0xef) >> 4i32, ubig!(0xe));
    assert_eq!(ubig!(0xef) >> &4i32, ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> 4i32, ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> &4i32, ubig!(0xe));

    assert_eq!(ubig!(0xef) >> ibig!(4), ubig!(0xe));
    assert_eq!(ubig!(0xef) >> &ibig!(4), ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> ibig!(4), ubig!(0xe));
    assert_eq!(&ubig!(0xef) >> &ibig!(4), ubig!(0xe));
}

#[test]
#[should_panic]
fn test_ubig_shr_negative() {
    let _ = ubig!(1) >> (-3i32);
}

#[test]
fn test_ubig_shr_assign() {
    let mut a = ubig!(0xeff);
    a >>= 4;
    assert_eq!(a, ubig!(0xef));
    a >>= &4;
    assert_eq!(a, ubig!(0xe));
}

#[test]
fn test_ibig_shr() {
    let test_cases = [
        (ibig!(0xe0), 4, ibig!(0xe)),
        (ibig!(0xef), 4, ibig!(0xe)),
        (ibig!(0xef), 100, ibig!(0)),
        (ibig!(-0xe0), 4, ibig!(-0xe)),
        (ibig!(-0xef), 4, ibig!(-0xf)),
        (ibig!(-0xef), 100, ibig!(-1)),
        (ibig!(0xff) << 1000, 1000, ibig!(0xff)),
        ((ibig!(0xff) << 1000) + ibig!(1), 1000, ibig!(0xff)),
        (ibig!(-0xff) << 1000, 1000, ibig!(-0xff)),
        ((ibig!(-0xff) << 1000) - ibig!(1), 1000, ibig!(-0x100)),
        (
            (ibig!(-0xff) << 1000) - (ibig!(1) << 999),
            1000,
            ibig!(-0x100),
        ),
        (ibig!(-0xff) << 1000, 2000, ibig!(-1)),
    ];
    for (a, b, c) in &test_cases {
        assert_eq!(a >> b, *c);
        assert_eq!(a >> *b, *c);
        assert_eq!(a.clone() >> b, *c);
        assert_eq!(a.clone() >> *b, *c);

        let mut x = a.clone();
        x >>= b;
        assert_eq!(x, *c);

        let mut x = a.clone();
        x >>= *b;
        assert_eq!(x, *c);
    }

    assert_eq!(ibig!(0xef) >> (ubig!(1) << 1000), ibig!(0));
    assert_eq!(ibig!(-0xef) >> (ubig!(1) << 1000), ibig!(-1));
    assert_eq!(ibig!(-0xef) >> (ibig!(1) << 1000), ibig!(-1));
    assert_eq!(&ibig!(-0xef) >> (ibig!(1) << 1000), ibig!(-1));
    assert_eq!(ibig!(-0xef) >> &(ibig!(1) << 1000), ibig!(-1));
    assert_eq!(&ibig!(-0xef) >> &(ibig!(1) << 1000), ibig!(-1));
}
