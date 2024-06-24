#include <linux/ioctl.h>

typedef struct {
    char c;
} Size1;

typedef struct {
    char c[20];
} Size20;

typedef struct {
    char c[_IOC_SIZEMASK];
} SizeMax;

#define EQ_CHECK(rname, cname) \
    fprintf(fp, "    assert_eq!(%s, %u);\n", #rname, cname)


#define IOC_CHECK(dir, type, nr, size) \
    fprintf(fp, "    assert_eq!(ioc(%lu, %lu, %lu, %lu), 0x%lx);\n", \
            ((unsigned long) (dir)), \
            ((unsigned long) (type)), \
            ((unsigned long) (nr)), \
            ((unsigned long) (size)), \
            ((unsigned long) _IOC((dir), (type), (nr), (size))))

#define IO_CHECK(type, nr) \
    fprintf(fp, "    assert_eq!(io(%lu, %lu), 0x%lx);\n", \
            ((unsigned long) (type)), \
            ((unsigned long) (nr)), \
            ((unsigned long) _IO((type), (nr))))

#define IOFN_CHECK(rustfn, cfn, type, nr, passed_type) \
    fprintf(fp, "    assert_eq!(%s::<%s>(%lu, %lu), 0x%lx, \"Expected %s::<%s>(%lu, %lu) to equal 0x%lx\");\n", \
            #rustfn, \
            #passed_type, \
            ((unsigned long) (type)), \
            ((unsigned long) (nr)), \
            ((unsigned long) cfn((type), (nr), passed_type)), \
            #rustfn, \
            #passed_type, \
            ((unsigned long) (type)), \
            ((unsigned long) (nr)), \
            ((unsigned long) cfn((type), (nr), passed_type)))

#define IOFN_CHECKS(rustfn, cfn) \
    do { \
        IOFN_CHECK(ior, _IOR, 0, 0, Size1); \
        IOFN_CHECK(ior, _IOR, 1, 1, Size20); \
        IOFN_CHECK(ior, _IOR, 0, 0, SizeMax); \
        IOFN_CHECK(ior, _IOR, 0, _IOC_NRMASK, Size1); \
        IOFN_CHECK(ior, _IOR, 0, _IOC_NRMASK, SizeMax); \
        IOFN_CHECK(ior, _IOR, _IOC_TYPEMASK, 0, Size1); \
        IOFN_CHECK(ior, _IOR, _IOC_TYPEMASK, 0, SizeMax); \
        IOFN_CHECK(ior, _IOR, _IOC_TYPEMASK, _IOC_NRMASK, Size1); \
        IOFN_CHECK(ior, _IOR, _IOC_TYPEMASK, _IOC_NRMASK, SizeMax); \
    } while (0)

static int ioctl_ccompat_test(char const *arch) {
    char filename[256];
    FILE *fp;

    sprintf(filename, "src/ccompat_tests_linux_%s.rs", arch);
    fp = fopen(filename, "w");
    if (fp == NULL) {
        perror(filename);
        return 1;
    }

    fprintf(fp, "//! This is an automatically generated file; do not edit.\n");
    fprintf(fp, "//! Generated by gen_ccompat_tests.c on %s %s\n", __DATE__, __TIME__);
    fprintf(fp, "use super::*;\n");
    fprintf(fp, "\n");
    fprintf(fp, "#[repr(C)]\n");
    fprintf(fp, "struct Size1([u8; 1]);\n");
    fprintf(fp, "\n");
    fprintf(fp, "#[repr(C)]\n");
    fprintf(fp, "struct Size20([u8; 20]);\n");
    fprintf(fp, "\n");
    fprintf(fp, "#[repr(C)]\n");
    fprintf(fp, "struct SizeMax([u8; %u]);\n", _IOC_SIZEMASK);
    fprintf(fp, "\n");
    fprintf(fp, "#[test]\n");
    fprintf(fp, "fn test_ioctl_ccompat() {\n");
    fprintf(fp, "    assert_eq!(core::mem::size_of::<Size1>(), 1);\n");
    fprintf(fp, "    assert_eq!(core::mem::size_of::<Size20>(), 20);\n");
    fprintf(fp, "    assert_eq!(core::mem::size_of::<SizeMax>(), %u);\n", _IOC_SIZEMASK);
    fprintf(fp, "\n");
    EQ_CHECK(IOC_NRBITS, _IOC_NRBITS);
    EQ_CHECK(IOC_TYPEBITS, _IOC_TYPEBITS);
    EQ_CHECK(IOC_SIZEBITS, _IOC_SIZEBITS);
    EQ_CHECK(IOC_DIRBITS, _IOC_DIRBITS);
    EQ_CHECK(IOC_NONE, _IOC_NONE);
    EQ_CHECK(IOC_READ, _IOC_READ);
    EQ_CHECK(IOC_WRITE, _IOC_WRITE);
    EQ_CHECK(IOC_NRMASK, _IOC_NRMASK);
    EQ_CHECK(IOC_TYPEMASK, _IOC_TYPEMASK);
    EQ_CHECK(IOC_SIZEMASK, _IOC_SIZEMASK);
    EQ_CHECK(IOC_DIRMASK, _IOC_DIRMASK);
    EQ_CHECK(IOC_NRSHIFT, _IOC_NRSHIFT);
    EQ_CHECK(IOC_TYPESHIFT, _IOC_TYPESHIFT);
    EQ_CHECK(IOC_SIZESHIFT, _IOC_SIZESHIFT);
    EQ_CHECK(IOC_DIRSHIFT, _IOC_DIRSHIFT);

    IOC_CHECK(_IOC_NONE, 0, 0, 0);
    IOC_CHECK(_IOC_READ, 0, 0, 0);
    IOC_CHECK(_IOC_WRITE, 0, 0, 0);
    IOC_CHECK(_IOC_WRITE|_IOC_READ, 0, 0, 0);
    IOC_CHECK(_IOC_READ, 1, 1, 20);
    IOC_CHECK(_IOC_READ, 0, 0, _IOC_SIZEMASK);
    IOC_CHECK(_IOC_READ, 0, _IOC_NRMASK, 0);
    IOC_CHECK(_IOC_READ, 0, _IOC_NRMASK, _IOC_SIZEMASK);
    IOC_CHECK(_IOC_READ, _IOC_TYPEMASK, 0, 0);
    IOC_CHECK(_IOC_READ, _IOC_TYPEMASK, 0, _IOC_SIZEMASK);
    IOC_CHECK(_IOC_READ, _IOC_TYPEMASK, _IOC_NRMASK, 0);
    IOC_CHECK(_IOC_READ, _IOC_TYPEMASK, _IOC_NRMASK, _IOC_SIZEMASK);

    IO_CHECK(0, 0);
    IO_CHECK(1, 1);
    IO_CHECK(0, _IOC_NRMASK);
    IO_CHECK(_IOC_TYPEMASK, 0);
    IO_CHECK(_IOC_TYPEMASK, _IOC_NRMASK);

    IOFN_CHECKS(ior, _IOR);
    IOFN_CHECKS(iow, _IOW);
    IOFN_CHECKS(iowr, _IOWR);

    fprintf(fp, "}\n");
    fclose(fp);
    return 0;
}
