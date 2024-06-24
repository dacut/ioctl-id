//! This is an automatically generated file; do not edit.
//! Generated by gen_ccompat_tests.c on Jun 24 2024 06:40:29
use super::*;

#[repr(C)]
struct Size1([u8; 1]);

#[repr(C)]
struct Size20([u8; 20]);

#[repr(C)]
struct SizeMax([u8; 16383]);

#[test]
fn test_ioctl_ccompat() {
    assert_eq!(core::mem::size_of::<Size1>(), 1);
    assert_eq!(core::mem::size_of::<Size20>(), 20);
    assert_eq!(core::mem::size_of::<SizeMax>(), 16383);

    assert_eq!(IOC_NRBITS, 8);
    assert_eq!(IOC_TYPEBITS, 8);
    assert_eq!(IOC_SIZEBITS, 14);
    assert_eq!(IOC_DIRBITS, 2);
    assert_eq!(IOC_NONE, 0);
    assert_eq!(IOC_READ, 2);
    assert_eq!(IOC_WRITE, 1);
    assert_eq!(IOC_NRMASK, 255);
    assert_eq!(IOC_TYPEMASK, 255);
    assert_eq!(IOC_SIZEMASK, 16383);
    assert_eq!(IOC_DIRMASK, 3);
    assert_eq!(IOC_NRSHIFT, 0);
    assert_eq!(IOC_TYPESHIFT, 8);
    assert_eq!(IOC_SIZESHIFT, 16);
    assert_eq!(IOC_DIRSHIFT, 30);
    assert_eq!(ioc(0, 0, 0, 0), 0x0);
    assert_eq!(ioc(2, 0, 0, 0), 0x80000000);
    assert_eq!(ioc(1, 0, 0, 0), 0x40000000);
    assert_eq!(ioc(3, 0, 0, 0), 0xc0000000);
    assert_eq!(ioc(2, 1, 1, 20), 0x80140101);
    assert_eq!(ioc(2, 0, 0, 16383), 0xbfff0000);
    assert_eq!(ioc(2, 0, 255, 0), 0x800000ff);
    assert_eq!(ioc(2, 0, 255, 16383), 0xbfff00ff);
    assert_eq!(ioc(2, 255, 0, 0), 0x8000ff00);
    assert_eq!(ioc(2, 255, 0, 16383), 0xbfffff00);
    assert_eq!(ioc(2, 255, 255, 0), 0x8000ffff);
    assert_eq!(ioc(2, 255, 255, 16383), 0xbfffffff);
    assert_eq!(io(0, 0), 0x0);
    assert_eq!(io(1, 1), 0x101);
    assert_eq!(io(0, 255), 0xff);
    assert_eq!(io(255, 0), 0xff00);
    assert_eq!(io(255, 255), 0xffff);
    assert_eq!(ior::<Size1>(0, 0), 0x80010000, "Expected ior::<Size1>(0, 0) to equal 0x80010000");
    assert_eq!(ior::<Size20>(1, 1), 0x80140101, "Expected ior::<Size20>(1, 1) to equal 0x80140101");
    assert_eq!(ior::<SizeMax>(0, 0), 0xbfff0000, "Expected ior::<SizeMax>(0, 0) to equal 0xbfff0000");
    assert_eq!(ior::<Size1>(0, 255), 0x800100ff, "Expected ior::<Size1>(0, 255) to equal 0x800100ff");
    assert_eq!(ior::<SizeMax>(0, 255), 0xbfff00ff, "Expected ior::<SizeMax>(0, 255) to equal 0xbfff00ff");
    assert_eq!(ior::<Size1>(255, 0), 0x8001ff00, "Expected ior::<Size1>(255, 0) to equal 0x8001ff00");
    assert_eq!(ior::<SizeMax>(255, 0), 0xbfffff00, "Expected ior::<SizeMax>(255, 0) to equal 0xbfffff00");
    assert_eq!(ior::<Size1>(255, 255), 0x8001ffff, "Expected ior::<Size1>(255, 255) to equal 0x8001ffff");
    assert_eq!(ior::<SizeMax>(255, 255), 0xbfffffff, "Expected ior::<SizeMax>(255, 255) to equal 0xbfffffff");
    assert_eq!(ior::<Size1>(0, 0), 0x80010000, "Expected ior::<Size1>(0, 0) to equal 0x80010000");
    assert_eq!(ior::<Size20>(1, 1), 0x80140101, "Expected ior::<Size20>(1, 1) to equal 0x80140101");
    assert_eq!(ior::<SizeMax>(0, 0), 0xbfff0000, "Expected ior::<SizeMax>(0, 0) to equal 0xbfff0000");
    assert_eq!(ior::<Size1>(0, 255), 0x800100ff, "Expected ior::<Size1>(0, 255) to equal 0x800100ff");
    assert_eq!(ior::<SizeMax>(0, 255), 0xbfff00ff, "Expected ior::<SizeMax>(0, 255) to equal 0xbfff00ff");
    assert_eq!(ior::<Size1>(255, 0), 0x8001ff00, "Expected ior::<Size1>(255, 0) to equal 0x8001ff00");
    assert_eq!(ior::<SizeMax>(255, 0), 0xbfffff00, "Expected ior::<SizeMax>(255, 0) to equal 0xbfffff00");
    assert_eq!(ior::<Size1>(255, 255), 0x8001ffff, "Expected ior::<Size1>(255, 255) to equal 0x8001ffff");
    assert_eq!(ior::<SizeMax>(255, 255), 0xbfffffff, "Expected ior::<SizeMax>(255, 255) to equal 0xbfffffff");
    assert_eq!(ior::<Size1>(0, 0), 0x80010000, "Expected ior::<Size1>(0, 0) to equal 0x80010000");
    assert_eq!(ior::<Size20>(1, 1), 0x80140101, "Expected ior::<Size20>(1, 1) to equal 0x80140101");
    assert_eq!(ior::<SizeMax>(0, 0), 0xbfff0000, "Expected ior::<SizeMax>(0, 0) to equal 0xbfff0000");
    assert_eq!(ior::<Size1>(0, 255), 0x800100ff, "Expected ior::<Size1>(0, 255) to equal 0x800100ff");
    assert_eq!(ior::<SizeMax>(0, 255), 0xbfff00ff, "Expected ior::<SizeMax>(0, 255) to equal 0xbfff00ff");
    assert_eq!(ior::<Size1>(255, 0), 0x8001ff00, "Expected ior::<Size1>(255, 0) to equal 0x8001ff00");
    assert_eq!(ior::<SizeMax>(255, 0), 0xbfffff00, "Expected ior::<SizeMax>(255, 0) to equal 0xbfffff00");
    assert_eq!(ior::<Size1>(255, 255), 0x8001ffff, "Expected ior::<Size1>(255, 255) to equal 0x8001ffff");
    assert_eq!(ior::<SizeMax>(255, 255), 0xbfffffff, "Expected ior::<SizeMax>(255, 255) to equal 0xbfffffff");
}