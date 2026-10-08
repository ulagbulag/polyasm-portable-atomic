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
            $(all(any(miri, portable_atomic_sanitize_thread), $condition)),+
        ))]
        pub(super) mod intrinsics;

        $(
            #[cfg(all(any(miri, portable_atomic_sanitize_thread), $condition))]
            pub(super) use self::intrinsics as $backend;

            #[cfg(all(not(any(miri, portable_atomic_sanitize_thread)), $condition))]
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
                target_arch = "aarch64",
                not(all(
                    any(miri, portable_atomic_sanitize_thread),
                    not(portable_atomic_atomic_intrinsics),
                )),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            ),
            all(
                target_arch = "arm64ec",
                not(all(
                    any(miri, portable_atomic_sanitize_thread),
                    not(portable_atomic_atomic_intrinsics),
                )),
                not(portable_atomic_no_asm),
            ),
        )
    ),
    // powerpc64
    (
        powerpc64,
        all(
            target_arch = "powerpc64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
            any(
                target_feature = "quadword-atomics",
                portable_atomic_target_feature = "quadword-atomics",
                all(
                    feature = "fallback",
                    not(portable_atomic_no_outline_atomics),
                    any(
                        all(
                            target_os = "linux",
                            any(
                                all(
                                    target_env = "gnu",
                                    any(
                                        target_endian = "little",
                                        not(target_feature = "crt-static"),
                                    ),
                                ),
                                all(
                                    target_env = "musl",
                                    any(not(target_feature = "crt-static"), feature = "std"),
                                ),
                                target_env = "ohos",
                                all(target_env = "uclibc", not(target_feature = "crt-static")),
                                portable_atomic_outline_atomics,
                            ),
                        ),
                        target_os = "android",
                        all(
                            target_os = "freebsd",
                            any(
                                target_endian = "little",
                                not(target_feature = "crt-static"),
                                portable_atomic_outline_atomics,
                            ),
                        ),
                        target_os = "openbsd",
                        all(
                            target_os = "aix",
                            not(portable_atomic_pre_llvm_20),
                            any(test, portable_atomic_outline_atomics),
                        ), // aix: opt-in through portable_atomic_outline_atomics
                    ),
                    not(any(miri, portable_atomic_sanitize_thread)),
                ),
            ),
        )
    ),
    // s390x
    (
        s390x,
        all(
            target_arch = "s390x",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        )
    ),
    // x86_64
    (
        x86_64,
        all(
            target_arch = "x86_64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                portable_atomic_no_cmpxchg16b_intrinsic,
            )),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(
                target_feature = "cmpxchg16b",
                portable_atomic_target_feature = "cmpxchg16b",
                all(
                    feature = "fallback",
                    not(portable_atomic_no_outline_atomics),
                    not(any(target_env = "sgx", miri)),
                ),
            ),
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
