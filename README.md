<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

## [Interactive site](https://djkoloski.github.io/rust_serialization_benchmark/)

Calculate the number of messages per second that can be sent/received with various rust serialization frameworks and compression libraries.
[Documentation](pages/README.md)

## Format

All tests benchmark the following properties (time or size):

* **Serialize**: serialize data into a buffer
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression
* **Zstd**: the size of the buffer after zstd compression
* **Zstd Time**: the time taken to compress the serialized buffer with zstd

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2024-12-21 6:24:35

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.85.0-nightly (5f23ef7d3 2024-12-20)
binary: rustc
commit-hash: 5f23ef7d3f7a8c3e0ca5c4e1978829c0448a3686
commit-date: 2024-12-20
host: x86_64-unknown-linux-gnu
release: 1.85.0-nightly
LLVM version: 19.1.6
```

### CPU info

```
Architecture:                       x86_64
CPU op-mode(s):                     32-bit, 64-bit
Address sizes:                      48 bits physical, 48 bits virtual
Byte Order:                         Little Endian
CPU(s):                             4
On-line CPU(s) list:                0-3
Vendor ID:                          AuthenticAMD
Model name:                         AMD EPYC 7763 64-Core Processor
CPU family:                         25
Model:                              1
Thread(s) per core:                 2
Core(s) per socket:                 2
Socket(s):                          1
Stepping:                           1
BogoMIPS:                           4890.86
Flags:                              fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                     AMD-V
Hypervisor vendor:                  Microsoft
Virtualization type:                full
L1d cache:                          64 KiB (2 instances)
L1i cache:                          64 KiB (2 instances)
L2 cache:                           1 MiB (2 instances)
L3 cache:                           32 MiB (1 instance)
NUMA node(s):                       1
NUMA node0 CPU(s):                  0-3
Vulnerability Gather data sampling: Not affected
Vulnerability Itlb multihit:        Not affected
Vulnerability L1tf:                 Not affected
Vulnerability Mds:                  Not affected
Vulnerability Meltdown:             Not affected
Vulnerability Mmio stale data:      Not affected
Vulnerability Retbleed:             Not affected
Vulnerability Spec rstack overflow: Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:    Vulnerable
Vulnerability Spectre v1:           Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:           Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                Not affected
Vulnerability Tsx async abort:      Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*712.40 µs\**</span> <span title="prepend">*617.34 µs\**</span> | 3.1748 ms | 874632 | 355446 | 311723 | 5.1105 ms |
| [bincode 2.0.0-rc][bincode] | 335.42 µs | 2.5110 ms | 741295 | 303944 | 257153 | 3.9848 ms |
| [bincode 1.3.3][bincode1] | 516.38 µs | 1.9825 ms | 1045784 | 373127 | 311761 | 4.8160 ms |
| [bitcode 0.6.3][bitcode] | 137.34 µs | 1.4743 ms | 703710 | 288826 | 229755 | 2.4164 ms |
| [borsh 1.5.1][borsh] | 543.16 µs | 2.2195 ms | 885780 | 362204 | 286514 | 4.5701 ms |
| [capnp 0.19.7][capnp] | 519.85 µs | † | 1443216 | 513986 | 428649 | 6.7818 ms |
| [cbor4ii 0.3.3][cbor4ii] | 593.84 µs | 4.9615 ms | 1407835 | 403440 | 324081 | 5.2553 ms |
| [ciborium 0.2.2][ciborium] | 3.1514 ms | 11.981 ms | 1407835 | 403440 | 324081 | 4.8302 ms |
| [databuf 0.5.0][databuf] | 264.72 µs | 2.0926 ms | 765778 | 311715 | 264630 | 4.1563 ms |
| [dlhn 0.1.7][dlhn] | 728.81 µs | 2.5387 ms | 724953 | 301446 | 253629 | 3.8360 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0720 ms | † | 1276368 | 468539 | 388832 | 5.4571 ms |
| [msgpacker 0.4.3][msgpacker] | 1.3675 ms | 2.5976 ms | 764996 | 315291 | 264898 | 4.2215 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5040 ms | 4.1085 ms | 818669 | 332556 | 285514 | 4.6385 ms |
| [nanoserde 0.1.37][nanoserde] | 266.88 µs | 2.0882 ms | 1045784 | 373127 | 311761 | 4.5160 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 661.92 µs | 2.2104 ms | 765778 | 311743 | 264518 | 4.1870 ms |
| [postcard 1.0.10][postcard] | 424.59 µs | 2.2086 ms | 724953 | 302399 | 253747 | 3.8191 ms |
| [pot 3.0.1][pot] | 2.2602 ms | 6.3980 ms | 971922 | 372513 | 304122 | 4.9162 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.0635 ms\**</span> <span title="populate + encode">*2.5742 ms\**</span> | 3.3608 ms | 884628 | 363130 | 315494 | 5.1624 ms |
| [rkyv 0.8.5][rkyv] | 247.43 µs | <span title="unvalidated">*1.5865 ms\**</span> <span title="validated upfront with error">*2.2107 ms\**</span> | 1011488 | 393526 | 326517 | 4.9529 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3161 ms | 3.1685 ms | 784997 | 325384 | 278219 | 4.1500 ms |
| [ron 0.8.1][ron] | 11.369 ms | 16.030 ms | 1607459 | 449158 | 349713 | 5.6469 ms |
| [savefile 0.17.7][savefile] | 188.90 µs | 2.2065 ms | 1045800 | 373140 | 311777 | 4.5428 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5552 ms | 4.7233 ms | 1584946 | 413733 | 341439 | 4.9262 ms |
| [serde_bare 0.5.0][serde_bare] | 661.68 µs | 2.1325 ms | 765778 | 311715 | 264630 | 3.8593 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9755 ms | 4.7679 ms | 1407835 | 403440 | 324081 | 4.8007 ms |
| [serde_json 1.0.128][serde_json] | 3.7851 ms | 5.3636 ms | 1827461 | 470560 | 361090 | 5.5844 ms |
| [serialization 0.2.9][serialization] | 231.19 µs | 1.7149 ms | 765778 | 305641 | 259060 | 4.0778 ms |
| [speedy 0.8.7][speedy] | 199.67 µs | 1.7911 ms | 885780 | 362204 | 286514 | 4.2249 ms |
| [wiring 0.2.2][wiring] | 190.39 µs | 1.9847 ms | 1045784 | 337930 | 276188 | 5.2529 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*81.748 ns\**</span> | <span title="validated on-demand with error">*162.70 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4741 ns\**</span> <span title="validated upfront with error">*1.9853 ms\**</span> | <span title="unvalidated">*51.449 µs\**</span> <span title="validated upfront with error">*2.0081 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2361 ns\**</span> <span title="validated upfront with error">*642.08 µs\**</span> | <span title="unvalidated">*10.506 µs\**</span> <span title="validated upfront with error">*646.56 µs\**</span> | <span title="unvalidated">*7.7376 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*19.28%\**</span> <span title="prepend">*22.25%\**</span> | 46.44% | 80.46% | 81.26% | 73.70% | 47.28% |
| [bincode 2.0.0-rc][bincode] | 40.95% | 58.71% | 94.93% | 95.03% | 89.35% | 60.64% |
| [bincode 1.3.3][bincode1] | 26.60% | 74.37% | 67.29% | 77.41% | 73.70% | 50.17% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.29% | 66.42% | 79.45% | 79.74% | 80.19% | 52.87% |
| [capnp 0.19.7][capnp] | 26.42% | † | 48.76% | 56.19% | 53.60% | 35.63% |
| [cbor4ii 0.3.3][cbor4ii] | 23.13% | 29.71% | 49.99% | 71.59% | 70.89% | 45.98% |
| [ciborium 0.2.2][ciborium] | 4.36% | 12.31% | 49.99% | 71.59% | 70.89% | 50.03% |
| [databuf 0.5.0][databuf] | 51.88% | 70.45% | 91.89% | 92.66% | 86.82% | 58.14% |
| [dlhn 0.1.7][dlhn] | 18.84% | 58.07% | 97.07% | 95.81% | 90.59% | 62.99% |
| [flatbuffers 24.3.25][flatbuffers] | 12.81% | † | 55.13% | 61.64% | 59.09% | 44.28% |
| [msgpacker 0.4.3][msgpacker] | 10.04% | 56.76% | 91.99% | 91.61% | 86.73% | 57.24% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.50% | 35.88% | 85.96% | 86.85% | 80.47% | 52.09% |
| [nanoserde 0.1.37][nanoserde] | 51.46% | 70.60% | 67.29% | 77.41% | 73.70% | 53.51% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.75% | 66.70% | 91.89% | 92.65% | 86.86% | 57.71% |
| [postcard 1.0.10][postcard] | 32.35% | 66.75% | 97.07% | 95.51% | 90.54% | 63.27% |
| [pot 3.0.1][pot] | 6.08% | 23.04% | 72.40% | 77.53% | 75.55% | 49.15% |
| [prost 0.13.2][prost] | <span title="encode">*12.91%\**</span> <span title="populate + encode">*5.34%\**</span> | 43.87% | 79.55% | 79.54% | 72.82% | 46.81% |
| [rkyv 0.8.5][rkyv] | 55.51% | <span title="unvalidated">*92.93%\**</span> <span title="validated upfront with error">*66.69%\**</span> | 69.57% | 73.39% | 70.37% | 48.79% |
| [rmp-serde 1.3.0][rmp-serde] | 10.44% | 46.53% | 89.64% | 88.76% | 82.58% | 58.23% |
| [ron 0.8.1][ron] | 1.21% | 9.20% | 43.78% | 64.30% | 65.70% | 42.79% |
| [savefile 0.17.7][savefile] | 72.71% | 66.82% | 67.29% | 77.40% | 73.69% | 53.19% |
| [serde-brief 0.1.0][serde-brief] | 8.83% | 31.21% | 44.40% | 69.81% | 67.29% | 49.05% |
| [serde_bare 0.5.0][serde_bare] | 20.76% | 69.13% | 91.89% | 92.66% | 86.82% | 62.61% |
| [serde_cbor 0.11.2][serde_cbor] | 6.95% | 30.92% | 49.99% | 71.59% | 70.89% | 50.33% |
| [serde_json 1.0.128][serde_json] | 3.63% | 27.49% | 38.51% | 61.38% | 63.63% | 43.27% |
| [serialization 0.2.9][serialization] | 59.41% | 85.97% | 91.89% | 94.50% | 88.69% | 59.26% |
| [speedy 0.8.7][speedy] | 68.78% | 82.31% | 79.45% | 79.74% | 80.19% | 57.19% |
| [wiring 0.2.2][wiring] | 72.14% | 74.28% | 67.29% | 85.47% | 83.19% | 46.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.51%\**</span> | <span title="validated on-demand with error">*6.46%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.42%\**</span> <span title="validated upfront with error">*0.52%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.62%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6873 ms\**</span> <span title="prepend">*8.7930 ms\**</span> | 8.7403 ms | 8625005 | 6443961 | 6231572 | 70.490 ms |
| [bincode 2.0.0-rc][bincode] | 2.3969 ms | 1.0165 ms | 6000005 | 5378497 | 5345897 | 7.7646 ms |
| [bincode 1.3.3][bincode1] | 5.1351 ms | 4.7051 ms | 6000008 | 5378500 | 5345890 | 7.3690 ms |
| [bitcode 0.6.3][bitcode] | 1.3977 ms | 796.03 µs | 6000006 | 5182295 | 4923880 | 12.886 ms |
| [borsh 1.5.1][borsh] | 5.6612 ms | 4.1198 ms | 6000004 | 5378496 | 5345889 | 7.3532 ms |
| [capnp 0.19.7][capnp] | 5.4652 ms | † | 14000088 | 7130367 | 6051062 | 82.556 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.8661 ms | 51.064 ms | 13125016 | 7524114 | 6757967 | 88.063 ms |
| [ciborium 0.2.2][ciborium] | 67.266 ms | 116.62 ms | 13122324 | 7524660 | 6759658 | 87.984 ms |
| [databuf 0.5.0][databuf] | 2.3939 ms | 5.3102 ms | 6000003 | 5378495 | 5345900 | 7.3712 ms |
| [dlhn 0.1.7][dlhn] | 6.5034 ms | 7.0994 ms | 6000003 | 5378495 | 5345900 | 7.3964 ms |
| [flatbuffers 24.3.25][flatbuffers] | 863.53 µs | † | 6000024 | 5378434 | 5345910 | 7.4175 ms |
| [msgpacker 0.4.3][msgpacker] | 18.396 ms | 5.1577 ms | 7500005 | 6058442 | 6014337 | 9.8461 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.91 ms | 32.351 ms | 8125037 | 6493484 | 6386940 | 71.471 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2467 ms | 1.0982 ms | 6000008 | 5378500 | 5345890 | 7.6507 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0733 ms | 3.9931 ms | 6000004 | 5378496 | 5345889 | 7.5403 ms |
| [postcard 1.0.10][postcard] | 509.57 µs | 1.2178 ms | 6000003 | 5378495 | 5345900 | 7.3980 ms |
| [pot 3.0.1][pot] | 38.976 ms | 71.705 ms | 10122342 | 6814618 | 6852251 | 78.231 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7366 ms\**</span> <span title="populate + encode">*8.2467 ms\**</span> | 14.418 ms | 8750000 | 6665735 | 6421871 | 70.406 ms |
| [rkyv 0.8.5][rkyv] | 237.31 µs | <span title="unvalidated">*148.69 µs\**</span> <span title="validated upfront with error">*148.65 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5485 ms |
| [rmp-serde 1.3.0][rmp-serde] | 14.480 ms | 17.241 ms | 8125006 | 6494876 | 6391037 | 67.557 ms |
| [ron 0.8.1][ron] | 171.91 ms | 235.27 ms | 22192885 | 8970395 | 8138755 | 146.67 ms |
| [savefile 0.17.7][savefile] | 237.30 µs | 236.96 µs | 6000024 | 5378513 | 5345893 | 7.9692 ms |
| [serde-brief 0.1.0][serde-brief] | 22.527 ms | 41.087 ms | 15750015 | 8024540 | 6816643 | 90.497 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2818 ms | 4.7314 ms | 6000003 | 5378495 | 5345900 | 7.7606 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.410 ms | 47.145 ms | 13122324 | 7524660 | 6759658 | 87.089 ms |
| [serde_json 1.0.128][serde_json] | 87.339 ms | 83.929 ms | 26192883 | 9566084 | 8586741 | 151.10 ms |
| [serialization 0.2.9][serialization] | 342.79 µs | 224.87 µs | 6000003 | 5378495 | 5345900 | 7.4187 ms |
| [speedy 0.8.7][speedy] | 237.26 µs | 238.39 µs | 6000004 | 5378496 | 5345889 | 7.5491 ms |
| [wiring 0.2.2][wiring] | 197.06 µs | 316.97 µs | 6000008 | 5378952 | 5345894 | 7.3916 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*118.01 ns\**</span> | <span title="validated on-demand with error">*2.1300 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*39.380 ns\**</span> | <span title="unvalidated">*54.168 µs\**</span> <span title="validated upfront with error">*77.437 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2358 ns\**</span> <span title="validated upfront with error">*5.2695 ns\**</span> | <span title="unvalidated">*48.438 µs\**</span> <span title="validated upfront with error">*38.704 µs\**</span> | <span title="unvalidated">*79.316 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.95%\**</span> <span title="prepend">*2.24%\**</span> | 1.70% | 69.57% | 80.42% | 79.02% | 10.43% |
| [bincode 2.0.0-rc][bincode] | 8.22% | 14.62% | 100.00% | 96.35% | 92.11% | 94.70% |
| [bincode 1.3.3][bincode1] | 3.84% | 3.16% | 100.00% | 96.35% | 92.11% | 99.79% |
| [bitcode 0.6.3][bitcode] | 14.10% | 18.67% | 100.00% | 100.00% | 100.00% | 57.06% |
| [borsh 1.5.1][borsh] | 3.48% | 3.61% | 100.00% | 96.35% | 92.11% | 100.00% |
| [capnp 0.19.7][capnp] | 3.61% | † | 42.86% | 72.68% | 81.37% | 8.91% |
| [cbor4ii 0.3.3][cbor4ii] | 2.00% | 0.29% | 45.71% | 68.88% | 72.86% | 8.35% |
| [ciborium 0.2.2][ciborium] | 0.29% | 0.13% | 45.72% | 68.87% | 72.84% | 8.36% |
| [databuf 0.5.0][databuf] | 8.23% | 2.80% | 100.00% | 96.35% | 92.11% | 99.76% |
| [dlhn 0.1.7][dlhn] | 3.03% | 2.09% | 100.00% | 96.35% | 92.11% | 99.42% |
| [flatbuffers 24.3.25][flatbuffers] | 22.82% | † | 100.00% | 96.35% | 92.11% | 99.13% |
| [msgpacker 0.4.3][msgpacker] | 1.07% | 2.88% | 80.00% | 85.54% | 81.87% | 74.68% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.46% | 73.85% | 79.81% | 77.09% | 10.29% |
| [nanoserde 0.1.37][nanoserde] | 15.81% | 13.54% | 100.00% | 96.35% | 92.11% | 96.11% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.88% | 3.72% | 100.00% | 96.35% | 92.11% | 97.52% |
| [postcard 1.0.10][postcard] | 38.67% | 12.21% | 100.00% | 96.35% | 92.11% | 99.39% |
| [pot 3.0.1][pot] | 0.51% | 0.21% | 59.27% | 76.05% | 71.86% | 9.40% |
| [prost 0.13.2][prost] | <span title="encode">*2.55%\**</span> <span title="populate + encode">*2.39%\**</span> | 1.03% | 68.57% | 77.75% | 76.67% | 10.44% |
| [rkyv 0.8.5][rkyv] | 83.04% | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 97.41% |
| [rmp-serde 1.3.0][rmp-serde] | 1.36% | 0.86% | 73.85% | 79.79% | 77.04% | 10.88% |
| [ron 0.8.1][ron] | 0.11% | 0.06% | 27.04% | 57.77% | 60.50% | 5.01% |
| [savefile 0.17.7][savefile] | 83.04% | 62.73% | 100.00% | 96.35% | 92.11% | 92.27% |
| [serde-brief 0.1.0][serde-brief] | 0.87% | 0.36% | 38.10% | 64.58% | 72.23% | 8.13% |
| [serde_bare 0.5.0][serde_bare] | 3.14% | 3.14% | 100.00% | 96.35% | 92.11% | 94.75% |
| [serde_cbor 0.11.2][serde_cbor] | 0.56% | 0.32% | 45.72% | 68.87% | 72.84% | 8.44% |
| [serde_json 1.0.128][serde_json] | 0.23% | 0.18% | 22.91% | 54.17% | 57.34% | 4.87% |
| [serialization 0.2.9][serialization] | 57.49% | 66.10% | 100.00% | 96.35% | 92.11% | 99.12% |
| [speedy 0.8.7][speedy] | 83.06% | 62.36% | 100.00% | 96.35% | 92.11% | 97.40% |
| [wiring 0.2.2][wiring] | 100.00% | 46.90% | 100.00% | 96.34% | 92.11% | 99.48% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.05%\**</span> | <span title="validated on-demand with error">*1.82%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*3.14%\**</span> | <span title="unvalidated">*71.45%\**</span> <span title="validated upfront with error">*49.98%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.45%\**</span> | <span title="unvalidated">*79.90%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.9][serialization] | 353.52 µs | † | † | † | † | † |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.9][serialization] | 100.00% | † | † | † | † | † |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|

[bilrost]: https://crates.io/crates/bilrost/0.1010.0
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.3
[borsh]: https://crates.io/crates/borsh/1.5.1
[capnp]: https://crates.io/crates/capnp/0.19.7
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.3
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/24.3.25
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.12
[postcard]: https://crates.io/crates/postcard/1.0.10
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.2
[rkyv]: https://crates.io/crates/rkyv/0.8.5
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.17.7
[serde-brief]: https://crates.io/crates/serde-brief/0.1.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.128
[serialization]: https://crates.io/crates/serialization/0.2.9
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
