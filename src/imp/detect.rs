// SPDX-License-Identifier: Apache-2.0 OR MIT

mod common;

#[cfg(all(
    any(target_arch = "aarch64", target_arch = "arm64ec"),
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ),
))]
pub(crate) mod aarch64_aa64reg;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_vendor = "apple",))]
pub(crate) mod aarch64_apple;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_os = "fuchsia",))]
pub(crate) mod aarch64_fuchsia;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_os = "illumos",))]
pub(crate) mod aarch64_illumos;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), windows))]
pub(crate) mod aarch64_windows;
#[cfg(all(
    any(target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64"),
    any(target_os = "linux", target_os = "android", target_os = "freebsd", target_os = "openbsd",),
))]
pub(crate) mod auxv;
#[cfg(all(target_arch = "powerpc64", target_os = "aix"))]
pub(crate) mod powerpc64_aix;
#[cfg(all(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    any(target_os = "linux", target_os = "android"),
))]
pub(crate) mod riscv_linux;
#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;

#[cfg(all(
    any(target_arch = "aarch64", target_arch = "arm64ec"),
    any(target_os = "netbsd", target_os = "openbsd"),
))]
pub(crate) use self::aarch64_aa64reg::detect;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_vendor = "apple",))]
pub(crate) use self::aarch64_apple::detect;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_os = "fuchsia",))]
pub(crate) use self::aarch64_fuchsia::detect;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_os = "illumos",))]
pub(crate) use self::aarch64_illumos::detect;
#[cfg(all(any(target_arch = "aarch64", target_arch = "arm64ec"), windows))]
pub(crate) use self::aarch64_windows::detect;
#[cfg(all(target_arch = "arm", not(target_arch = "aarch64")))]
pub(crate) use self::auxv::detect;
#[cfg(all(
    any(target_arch = "aarch64", target_arch = "arm64ec"),
    any(target_os = "linux", target_os = "android", target_os = "freebsd"),
))]
pub(crate) use self::auxv::detect;
#[cfg(all(
    target_arch = "powerpc64",
    any(target_os = "linux", target_os = "android", target_os = "freebsd", target_os = "openbsd",),
))]
pub(crate) use self::auxv::detect;
#[cfg(all(target_arch = "powerpc64", target_os = "aix"))]
pub(crate) use self::powerpc64_aix::detect;
#[cfg(all(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    any(target_os = "linux", target_os = "android"),
))]
pub(crate) use self::riscv_linux::detect;
#[cfg(target_arch = "x86_64")]
pub(crate) use self::x86_64::detect;
