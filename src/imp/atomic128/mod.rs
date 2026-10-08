// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
128-bit atomic implementations on 64-bit architectures

See README.md for details.
*/

#[allow(unused_macros)]
#[macro_use]
mod macros;

// Miri and Sanitizer share one naturally owned intrinsic implementation. Each
// target keeps its established backend name through a cfg-selected re-export.
macro_rules! atomic128_backends {
    ($(($backend:ident, $condition:meta)),+ $(,)?) => {
        #[cfg(any(
            $(all($condition, any(miri, portable_atomic_sanitize_thread))),+
        ))]
        pub(super) mod intrinsics;

        $(
            #[cfg(all($condition, any(miri, portable_atomic_sanitize_thread)))]
            pub(super) use self::intrinsics as $backend;

            #[cfg(all($condition, not(any(miri, portable_atomic_sanitize_thread))))]
            pub(super) mod $backend;
        )+
    };
}

atomic128_backends!(
    // AArch64
    (
        aarch64,
        any(
            all(
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
                not(all(
                    any(miri, portable_atomic_sanitize_thread),
                    not(portable_atomic_atomic_intrinsics),
                )),
                target_arch = "aarch64",
            ),
            all(
                not(all(
                    any(miri, portable_atomic_sanitize_thread),
                    not(portable_atomic_atomic_intrinsics),
                )),
                not(portable_atomic_no_asm),
                target_arch = "arm64ec",
            ),
        )
    ),
    // powerpc64
    (
        powerpc64,
        all(
            any(
                all(
                    any(
                        all(
                            any(
                                all(
                                    any(feature = "std", not(target_feature = "crt-static")),
                                    target_env = "musl",
                                ),
                                all(
                                    any(
                                        not(target_feature = "crt-static"),
                                        target_endian = "little",
                                    ),
                                    target_env = "gnu",
                                ),
                                all(not(target_feature = "crt-static"), target_env = "uclibc"),
                                portable_atomic_outline_atomics,
                                target_env = "ohos",
                            ),
                            target_os = "linux",
                        ),
                        all(
                            any(
                                not(target_feature = "crt-static"),
                                portable_atomic_outline_atomics,
                                target_endian = "little",
                            ),
                            target_os = "freebsd",
                        ),
                        // aix: opt-in through portable_atomic_outline_atomics
                        all(
                            any(portable_atomic_outline_atomics, test),
                            not(portable_atomic_pre_llvm_20),
                            target_os = "aix",
                        ),
                        target_os = "android",
                        target_os = "openbsd",
                    ),
                    feature = "fallback",
                    not(any(miri, portable_atomic_sanitize_thread)),
                    not(portable_atomic_no_outline_atomics),
                ),
                portable_atomic_target_feature = "quadword-atomics",
                target_feature = "quadword-atomics",
            ),
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
            target_arch = "powerpc64",
        )
    ),
    // s390x
    (
        s390x,
        all(
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
            target_arch = "s390x",
        )
    ),
    // x86_64
    (
        x86_64,
        all(
            any(
                all(
                    feature = "fallback",
                    not(any(miri, target_env = "sgx")),
                    not(portable_atomic_no_outline_atomics),
                ),
                portable_atomic_target_feature = "cmpxchg16b",
                target_feature = "cmpxchg16b",
            ),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                portable_atomic_no_cmpxchg16b_intrinsic,
            )),
            target_arch = "x86_64",
        )
    ),
);

// riscv64
#[cfg(all(
    target_arch = "riscv64",
    not(any(miri, portable_atomic_sanitize_thread)),
    any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
    any(
        target_feature = "zacas",
        portable_atomic_target_feature = "zacas",
        all(
            feature = "fallback",
            not(portable_atomic_no_outline_atomics),
            any(target_os = "linux", target_os = "android"),
        ),
    ),
))]
pub(super) mod riscv64;
