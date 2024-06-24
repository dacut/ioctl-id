#include <sys/utsname.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

static int ioctl_ccompat_test(const char *arch);

int main() {
    struct utsname name;

    if (uname(&name) != 0) {
        perror("uname");
        return 1;
    }

    if (strcmp(name.sysname, "Linux") == 0) {
        if (strncmp(name.machine, "ppc", 3) == 0) {
            return ioctl_ccompat_test("powerpc");
        } else if (strncmp(name.machine, "sparc", 5) == 0) {
            return ioctl_ccompat_test("sparc");
        } else {
            return ioctl_ccompat_test("generic");
        }
    } else if (strcmp(name.sysname, "Darwin") == 0) {
        return ioctl_ccompat_test(name.machine);
    } else {
        fprintf(stderr, "Unsupported OS: %s\n", name.sysname);
        return 1;
    }
}

#ifdef __linux__
#include "gen_ccompat_tests_linux.h"
#elif defined(__APPLE__) && defined(__MACH__)
#include "gen_ccompat_tests_macos.h"
#else
#error Unsupported OS
#endif