// SPDX-License-Identifier: Apache-2.0 OR MIT

mod common;

#[cfg(all(
    any(target_arch = "aarch64", target_arch = "arm64ec"),
    any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "linux",
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
    any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc64"),
    any(target_os = "android", target_os = "freebsd", target_os = "linux", target_os = "openbsd",),
))]
pub(crate) mod auxv;
#[cfg(all(target_arch = "powerpc64", target_os = "aix"))]
pub(crate) mod powerpc64_aix;
#[cfg(all(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    any(target_os = "android", target_os = "linux"),
))]
pub(crate) mod riscv_linux;
#[cfg(target_arch = "x86_64")]
pub(crate) mod x86_64;

#[inline]
pub(crate) fn detect() -> self::common::CpuInfo {
    cfg_select! {
        all(
            any(target_arch = "aarch64", target_arch = "arm64ec"),
            any(target_os = "netbsd", target_os = "openbsd")
        ) => self::aarch64_aa64reg::detect(),
        all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_vendor = "apple") => {
            self::aarch64_apple::detect()
        }
        all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_os = "fuchsia") => {
            self::aarch64_fuchsia::detect()
        }
        all(any(target_arch = "aarch64", target_arch = "arm64ec"), target_os = "illumos") => {
            self::aarch64_illumos::detect()
        }
        all(any(target_arch = "aarch64", target_arch = "arm64ec"), windows) => {
            self::aarch64_windows::detect()
        }
        all(not(target_arch = "aarch64"), target_arch = "arm") => self::auxv::detect(),
        all(
            any(target_arch = "aarch64", target_arch = "arm64ec"),
            any(target_os = "android", target_os = "freebsd", target_os = "linux")
        ) => self::auxv::detect(),
        all(
            any(
                target_os = "android",
                target_os = "freebsd",
                target_os = "linux",
                target_os = "openbsd"
            ),
            target_arch = "powerpc64"
        ) => self::auxv::detect(),
        all(target_arch = "powerpc64", target_os = "aix") => self::powerpc64_aix::detect(),
        all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            any(target_os = "android", target_os = "linux")
        ) => self::riscv_linux::detect(),
        target_arch = "x86_64" => self::x86_64::detect(),
    }
}
