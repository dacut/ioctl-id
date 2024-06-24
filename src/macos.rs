//! macOS-specific definitions.
//! 
//! On macOS, `ioctl(2)`identifiers are the size of C's `unsigned long` type,
//! equivalent to `u32` on 32-bit architectures and `u64` on 64-bit
//! architectures.
//! 
//! The `ioctl(2)` identifier is divided into three fields. From most-significant to least-significant in the identifier:
//! * The direction of the `ioctl(2)` call.
//! * The size of the structure passed to the `ioctl(2)` call.
//! * A group identifier.
//! * A call identifier.

use core::{ffi::c_ulong, mem::size_of};

/// The identifier type for ioctl calls.
pub type IoctlId = c_ulong;

/// A mask for the size of a parameter to the `ioctl(2)` call.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOCPARM_MASK: IoctlId = 0x1FFF;

/// The size of the parameter to the `ioctl(2)` call from its identifier.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn iocparm_len(value: IoctlId) -> IoctlId {
    (value >> 16) & IOCPARM_MASK
}

/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn iocbase_cmd(value: IoctlId) -> IoctlId {
    value & !(IOCPARM_MASK << 16)
}

/// Maximum size of `ioctl(2)` arguments.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOCPARM_MAX: usize = IOCPARM_MASK as usize + 1;

/// The `ioctl(2)` request takes no parameters.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOC_VOID: IoctlId = 0x20000000;

/// The `ioctl(2)` request receives parameters from the kernel.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOC_OUT: IoctlId = 0x40000000;

/// The `ioctl(2)` request sends parameters to the kernel.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOC_IN: IoctlId = 0x80000000;

/// The `ioctl(2)` request sends and receives parameters to and from the kernel.
/// 
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOC_INOUT: IoctlId = IOC_IN | IOC_OUT;

/// Mask for the direction values.
///
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const IOC_DIRMASK: IoctlId = 0xe0000000;

/// Create an `ioctl(2)` identifier from a direction value, group identifier,
/// call identifier, and size value.
/// 
/// This matches the definition of the `_IOC()` macro from the C headers.
///
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn ioc(inout: IoctlId, group: IoctlId, num: IoctlId, size: IoctlId) -> IoctlId {
    inout | ((size & IOCPARM_MASK) << 16) | (group << 8) | num
}

/// Create an `ioctl(2)` identifier for a call that passes no data.
///
/// This is the equivalent of the `_IO()` macro from the C headers.
///
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn io(group: IoctlId, num: IoctlId) -> IoctlId {
    ioc(IOC_VOID, group, num, 0)
}

/// Create an `ioctl(2)` identifier for a call that reads data.
///
/// This is the equivalent of the `_IOR()` macro from the C headers. The type must be passed as a generic parameter
/// to this function.
///
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn ior<T>(group: IoctlId, num: IoctlId) -> IoctlId {
    ioc(IOC_OUT, group, num, size_of::<T>() as IoctlId)
}

/// Create an `ioctl(2)` identifier for a call that writes data.
/// 
/// This is the equivalent of the `_IOW()` macro from the C headers. The type must be passed as a generic parameter
/// to this function.
///
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn iow<T>(group: IoctlId, num: IoctlId) -> IoctlId {
    ioc(IOC_IN, group, num, size_of::<T>() as IoctlId)
}

/// Create an `ioctl(2)` identifier for a call that reads and writes data.
///
/// This is the equivalent of the `_IOWR()` macro from the C headers. The type must be passed as a generic parameter
/// to this function.
///
/// Reference: [`sys/ioccom.h`](https://opensource.apple.com/source/xnu/xnu-201.5/bsd/sys/ioccom.h.auto.html)
pub const fn iowr<T>(group: IoctlId, num: IoctlId) -> IoctlId {
    ioc(IOC_INOUT, group, num, size_of::<T>() as IoctlId)
}