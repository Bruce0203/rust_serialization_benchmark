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

## Last updated: 2024-12-16 11:5:31

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.85.0-nightly (c26db435b 2024-12-15)
binary: rustc
commit-hash: c26db435bf8aee2efc397aab50f3a21eb351d6e5
commit-date: 2024-12-15
host: x86_64-unknown-linux-gnu
release: 1.85.0-nightly
LLVM version: 19.1.5
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
BogoMIPS:                           4890.85
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
| [bilrost 0.1010.0][bilrost] | <span title="encode">*694.84 µs\**</span> <span title="prepend">*618.01 µs\**</span> | 3.1222 ms | 874632 | 355446 | 311723 | 5.1304 ms |
| [bincode 2.0.0-rc][bincode] | 329.91 µs | 2.5200 ms | 741295 | 303944 | 257153 | 4.0291 ms |
| [bincode 1.3.3][bincode1] | 525.85 µs | 2.0209 ms | 1045784 | 373127 | 311761 | 4.8276 ms |
| [bitcode 0.6.3][bitcode] | 139.31 µs | 1.4888 ms | 703710 | 288826 | 229755 | 2.6046 ms |
| [borsh 1.5.1][borsh] | 542.78 µs | 2.2054 ms | 885780 | 362204 | 286514 | 4.6171 ms |
| [capnp 0.19.7][capnp] | 487.78 µs | † | 1443216 | 513986 | 428649 | 6.8195 ms |
| [cbor4ii 0.3.3][cbor4ii] | 588.02 µs | 4.9778 ms | 1407835 | 403440 | 324081 | 5.1241 ms |
| [ciborium 0.2.2][ciborium] | 3.2738 ms | 11.868 ms | 1407835 | 403440 | 324081 | 4.8459 ms |
| [databuf 0.5.0][databuf] | 269.27 µs | 2.0574 ms | 765778 | 311715 | 264630 | 3.8997 ms |
| [dlhn 0.1.7][dlhn] | 739.41 µs | 2.6517 ms | 724953 | 301446 | 253629 | 3.6356 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0174 ms | † | 1276368 | 468539 | 388832 | 5.2362 ms |
| [msgpacker 0.4.3][msgpacker] | 1.3640 ms | 2.5960 ms | 764996 | 315291 | 264898 | 4.2265 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4445 ms | 4.1695 ms | 818669 | 332556 | 285514 | 4.3760 ms |
| [nanoserde 0.1.37][nanoserde] | 301.34 µs | 2.1045 ms | 1045784 | 373127 | 311761 | 4.9230 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 646.70 µs | 2.2728 ms | 765778 | 311743 | 264518 | 4.2431 ms |
| [postcard 1.0.10][postcard] | 416.95 µs | 2.2216 ms | 724953 | 302399 | 253747 | 3.8525 ms |
| [pot 3.0.1][pot] | 2.2531 ms | 6.5798 ms | 971922 | 372513 | 304122 | 4.6255 ms |
| [prost 0.13.2][prost] | <span title="encode">*974.11 µs\**</span> <span title="populate + encode">*2.5246 ms\**</span> | 3.3299 ms | 884628 | 363130 | 315494 | 5.1419 ms |
| [rkyv 0.8.5][rkyv] | 244.58 µs | <span title="unvalidated">*1.5929 ms\**</span> <span title="validated upfront with error">*2.1795 ms\**</span> | 1011488 | 393526 | 326517 | 5.3017 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3896 ms | 3.1710 ms | 784997 | 325384 | 278219 | 4.4608 ms |
| [ron 0.8.1][ron] | 11.452 ms | 16.185 ms | 1607459 | 449158 | 349713 | 6.2449 ms |
| [savefile 0.17.7][savefile] | 191.99 µs | 2.1981 ms | 1045800 | 373140 | 311777 | 4.8719 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5310 ms | 5.1400 ms | 1584946 | 413733 | 341439 | 5.2087 ms |
| [serde_bare 0.5.0][serde_bare] | 670.71 µs | 2.0966 ms | 765778 | 311715 | 264630 | 4.1455 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7811 ms | 4.9544 ms | 1407835 | 403440 | 324081 | 5.2061 ms |
| [serde_json 1.0.128][serde_json] | 3.8527 ms | 5.4689 ms | 1827461 | 470560 | 361090 | 6.3905 ms |
| [serialization 0.2.7][serialization] | 414.14 µs | 1.5076 ms | 765778 | 305403 | 253430 | 4.0440 ms |
| [simd-json 0.13.10][simd-json] | 2.0685 ms | 4.6786 ms | 1827461 | 470560 | 361090 | 5.7863 ms |
| [speedy 0.8.7][speedy] | 198.79 µs | 1.8063 ms | 885780 | 362204 | 286514 | 4.3465 ms |
| [wiring 0.2.2][wiring] | 190.86 µs | 2.0000 ms | 1045784 | 337930 | 276188 | 4.2594 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*71.993 ns\**</span> | <span title="validated on-demand with error">*162.14 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4734 ns\**</span> <span title="validated upfront with error">*2.0591 ms\**</span> | <span title="unvalidated">*49.301 µs\**</span> <span title="validated upfront with error">*2.2599 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2373 ns\**</span> <span title="validated upfront with error">*588.67 µs\**</span> | <span title="unvalidated">*10.530 µs\**</span> <span title="validated upfront with error">*591.43 µs\**</span> | <span title="unvalidated">*7.5948 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*20.05%\**</span> <span title="prepend">*22.54%\**</span> | 47.68% | 80.46% | 81.26% | 73.70% | 50.77% |
| [bincode 2.0.0-rc][bincode] | 42.23% | 59.08% | 94.93% | 95.03% | 89.35% | 64.64% |
| [bincode 1.3.3][bincode1] | 26.49% | 73.67% | 67.29% | 77.41% | 73.70% | 53.95% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.67% | 67.51% | 79.45% | 79.74% | 80.19% | 56.41% |
| [capnp 0.19.7][capnp] | 28.56% | † | 48.76% | 56.19% | 53.60% | 38.19% |
| [cbor4ii 0.3.3][cbor4ii] | 23.69% | 29.91% | 49.99% | 71.59% | 70.89% | 50.83% |
| [ciborium 0.2.2][ciborium] | 4.26% | 12.54% | 49.99% | 71.59% | 70.89% | 53.75% |
| [databuf 0.5.0][databuf] | 51.74% | 72.36% | 91.89% | 92.66% | 86.82% | 66.79% |
| [dlhn 0.1.7][dlhn] | 18.84% | 56.15% | 97.07% | 95.81% | 90.59% | 71.64% |
| [flatbuffers 24.3.25][flatbuffers] | 13.69% | † | 55.13% | 61.64% | 59.09% | 49.74% |
| [msgpacker 0.4.3][msgpacker] | 10.21% | 57.35% | 91.99% | 91.61% | 86.73% | 61.63% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.56% | 35.71% | 85.96% | 86.85% | 80.47% | 59.52% |
| [nanoserde 0.1.37][nanoserde] | 46.23% | 70.74% | 67.29% | 77.41% | 73.70% | 52.91% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.54% | 65.51% | 91.89% | 92.65% | 86.86% | 61.38% |
| [postcard 1.0.10][postcard] | 33.41% | 67.01% | 97.07% | 95.51% | 90.54% | 67.61% |
| [pot 3.0.1][pot] | 6.18% | 22.63% | 72.40% | 77.53% | 75.55% | 56.31% |
| [prost 0.13.2][prost] | <span title="encode">*14.30%\**</span> <span title="populate + encode">*5.52%\**</span> | 44.71% | 79.55% | 79.54% | 72.82% | 50.65% |
| [rkyv 0.8.5][rkyv] | 56.96% | <span title="unvalidated">*93.46%\**</span> <span title="validated upfront with error">*68.31%\**</span> | 69.57% | 73.39% | 70.37% | 49.13% |
| [rmp-serde 1.3.0][rmp-serde] | 10.03% | 46.95% | 89.64% | 88.76% | 82.58% | 58.39% |
| [ron 0.8.1][ron] | 1.22% | 9.20% | 43.78% | 64.30% | 65.70% | 41.71% |
| [savefile 0.17.7][savefile] | 72.56% | 67.73% | 67.29% | 77.40% | 73.69% | 53.46% |
| [serde-brief 0.1.0][serde-brief] | 9.10% | 28.96% | 44.40% | 69.81% | 67.29% | 50.00% |
| [serde_bare 0.5.0][serde_bare] | 20.77% | 71.01% | 91.89% | 92.66% | 86.82% | 62.83% |
| [serde_cbor 0.11.2][serde_cbor] | 7.82% | 30.05% | 49.99% | 71.59% | 70.89% | 50.03% |
| [serde_json 1.0.128][serde_json] | 3.62% | 27.22% | 38.51% | 61.38% | 63.63% | 40.76% |
| [serialization 0.2.7][serialization] | 33.64% | 98.75% | 91.89% | 94.57% | 90.66% | 64.41% |
| [simd-json 0.13.10][simd-json] | 6.73% | 31.82% | 38.51% | 61.38% | 63.63% | 45.01% |
| [speedy 0.8.7][speedy] | 70.08% | 82.42% | 79.45% | 79.74% | 80.19% | 59.92% |
| [wiring 0.2.2][wiring] | 72.99% | 74.44% | 67.29% | 85.47% | 83.19% | 61.15% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*6.49%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.36%\**</span> <span title="validated upfront with error">*0.47%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.78%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6802 ms\**</span> <span title="prepend">*8.8428 ms\**</span> | 9.2312 ms | 8625005 | 6443961 | 6231572 | 69.773 ms |
| [bincode 2.0.0-rc][bincode] | 2.8711 ms | 1.0194 ms | 6000005 | 5378497 | 5345897 | 7.8719 ms |
| [bincode 1.3.3][bincode1] | 5.0932 ms | 6.8406 ms | 6000008 | 5378500 | 5345890 | 7.8542 ms |
| [bitcode 0.6.3][bitcode] | 1.5096 ms | 792.96 µs | 6000006 | 5182295 | 4923880 | 12.575 ms |
| [borsh 1.5.1][borsh] | 6.0893 ms | 4.3734 ms | 6000004 | 5378496 | 5345889 | 7.5407 ms |
| [capnp 0.19.7][capnp] | 5.3569 ms | † | 14000088 | 7130367 | 6051062 | 78.361 ms |
| [cbor4ii 0.3.3][cbor4ii] | 10.129 ms | 50.812 ms | 13125016 | 7524114 | 6757967 | 89.466 ms |
| [ciborium 0.2.2][ciborium] | 65.994 ms | 116.43 ms | 13122324 | 7524660 | 6759658 | 89.819 ms |
| [databuf 0.5.0][databuf] | 2.4006 ms | 5.3137 ms | 6000003 | 5378495 | 5345900 | 7.5061 ms |
| [dlhn 0.1.7][dlhn] | 6.2508 ms | 6.8138 ms | 6000003 | 5378495 | 5345900 | 7.4363 ms |
| [flatbuffers 24.3.25][flatbuffers] | 865.24 µs | † | 6000024 | 5378434 | 5345910 | 7.5509 ms |
| [msgpacker 0.4.3][msgpacker] | 18.834 ms | 5.2163 ms | 7500005 | 6058442 | 6014337 | 9.5538 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 124.45 ms | 32.954 ms | 8125037 | 6493484 | 6386940 | 67.391 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2798 ms | 1.1044 ms | 6000008 | 5378500 | 5345890 | 7.7187 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0774 ms | 4.5304 ms | 6000004 | 5378496 | 5345889 | 7.4745 ms |
| [postcard 1.0.10][postcard] | 490.94 µs | 2.1442 ms | 6000003 | 5378495 | 5345900 | 7.4398 ms |
| [pot 3.0.1][pot] | 37.985 ms | 75.061 ms | 10122342 | 6814618 | 6852251 | 78.812 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7630 ms\**</span> <span title="populate + encode">*9.0388 ms\**</span> | 14.938 ms | 8750000 | 6665735 | 6421871 | 70.019 ms |
| [rkyv 0.8.5][rkyv] | 238.58 µs | <span title="unvalidated">*148.79 µs\**</span> <span title="validated upfront with error">*149.84 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.3858 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.254 ms | 18.763 ms | 8125006 | 6494876 | 6391037 | 66.512 ms |
| [ron 0.8.1][ron] | 172.11 ms | 232.97 ms | 22192885 | 8970395 | 8138755 | 147.12 ms |
| [savefile 0.17.7][savefile] | 238.30 µs | 238.67 µs | 6000024 | 5378513 | 5345893 | 8.8516 ms |
| [serde-brief 0.1.0][serde-brief] | 23.171 ms | 36.247 ms | 15750015 | 8024540 | 6816643 | 90.978 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1805 ms | 4.7034 ms | 6000003 | 5378495 | 5345900 | 7.4470 ms |
| [serde_cbor 0.11.2][serde_cbor] | 32.784 ms | 47.233 ms | 13122324 | 7524660 | 6759658 | 88.644 ms |
| [serde_json 1.0.128][serde_json] | 88.317 ms | 84.191 ms | 26192883 | 9566084 | 8586741 | 151.10 ms |
| [serialization 0.2.7][serialization] | 2.3982 ms | 2.8912 ms | 6000003 | 5378944 | 5345890 | 7.4313 ms |
| [simd-json 0.13.10][simd-json] | 53.812 ms | 75.369 ms | 26192883 | 9566084 | 8586741 | 151.68 ms |
| [speedy 0.8.7][speedy] | 239.04 µs | 238.13 µs | 6000004 | 5378496 | 5345889 | 7.3871 ms |
| [wiring 0.2.2][wiring] | 147.92 µs | 317.80 µs | 6000008 | 5378952 | 5345894 | 7.4590 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*108.12 ns\**</span> | <span title="validated on-demand with error">*2.1873 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4815 ns\**</span> <span title="validated upfront with error">*39.432 ns\**</span> | <span title="unvalidated">*77.300 µs\**</span> <span title="validated upfront with error">*77.351 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2365 ns\**</span> <span title="validated upfront with error">*4.9573 ns\**</span> | <span title="unvalidated">*48.682 µs\**</span> <span title="validated upfront with error">*77.326 µs\**</span> | <span title="unvalidated">*77.334 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.21%\**</span> <span title="prepend">*1.67%\**</span> | 1.61% | 69.57% | 80.42% | 79.02% | 10.59% |
| [bincode 2.0.0-rc][bincode] | 5.15% | 14.60% | 100.00% | 96.35% | 92.11% | 93.82% |
| [bincode 1.3.3][bincode1] | 2.90% | 2.18% | 100.00% | 96.35% | 92.11% | 94.04% |
| [bitcode 0.6.3][bitcode] | 9.80% | 18.76% | 100.00% | 100.00% | 100.00% | 58.73% |
| [borsh 1.5.1][borsh] | 2.43% | 3.40% | 100.00% | 96.35% | 92.11% | 97.95% |
| [capnp 0.19.7][capnp] | 2.76% | † | 42.86% | 72.68% | 81.37% | 9.43% |
| [cbor4ii 0.3.3][cbor4ii] | 1.46% | 0.29% | 45.71% | 68.88% | 72.86% | 8.26% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.84% | 8.22% |
| [databuf 0.5.0][databuf] | 6.16% | 2.80% | 100.00% | 96.35% | 92.11% | 98.40% |
| [dlhn 0.1.7][dlhn] | 2.37% | 2.18% | 100.00% | 96.35% | 92.11% | 99.32% |
| [flatbuffers 24.3.25][flatbuffers] | 17.10% | † | 100.00% | 96.35% | 92.11% | 97.81% |
| [msgpacker 0.4.3][msgpacker] | 0.79% | 2.85% | 80.00% | 85.54% | 81.87% | 77.31% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.45% | 73.85% | 79.81% | 77.09% | 10.96% |
| [nanoserde 0.1.37][nanoserde] | 11.56% | 13.47% | 100.00% | 96.35% | 92.11% | 95.69% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.91% | 3.28% | 100.00% | 96.35% | 92.11% | 98.81% |
| [postcard 1.0.10][postcard] | 30.13% | 6.94% | 100.00% | 96.35% | 92.11% | 99.27% |
| [pot 3.0.1][pot] | 0.39% | 0.20% | 59.27% | 76.05% | 71.86% | 9.37% |
| [prost 0.13.2][prost] | <span title="encode">*1.91%\**</span> <span title="populate + encode">*1.64%\**</span> | 1.00% | 68.57% | 77.75% | 76.67% | 10.55% |
| [rkyv 0.8.5][rkyv] | 62.00% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.30%\**</span> | 100.00% | 96.35% | 92.11% | 100.00% |
| [rmp-serde 1.3.0][rmp-serde] | 0.97% | 0.79% | 73.85% | 79.79% | 77.04% | 11.10% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 5.02% |
| [savefile 0.17.7][savefile] | 62.07% | 62.34% | 100.00% | 96.35% | 92.11% | 83.44% |
| [serde-brief 0.1.0][serde-brief] | 0.64% | 0.41% | 38.10% | 64.58% | 72.23% | 8.12% |
| [serde_bare 0.5.0][serde_bare] | 2.39% | 3.16% | 100.00% | 96.35% | 92.11% | 99.18% |
| [serde_cbor 0.11.2][serde_cbor] | 0.45% | 0.32% | 45.72% | 68.87% | 72.84% | 8.33% |
| [serde_json 1.0.128][serde_json] | 0.17% | 0.18% | 22.91% | 54.17% | 57.34% | 4.89% |
| [serialization 0.2.7][serialization] | 6.17% | 5.15% | 100.00% | 96.34% | 92.11% | 99.39% |
| [simd-json 0.13.10][simd-json] | 0.27% | 0.20% | 22.91% | 54.17% | 57.34% | 4.87% |
| [speedy 0.8.7][speedy] | 61.88% | 62.48% | 100.00% | 96.35% | 92.11% | 99.98% |
| [wiring 0.2.2][wiring] | 100.00% | 46.82% | 100.00% | 96.34% | 92.11% | 99.02% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.14%\**</span> | <span title="validated on-demand with error">*2.23%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.83%\**</span> <span title="validated upfront with error">*3.14%\**</span> | <span title="unvalidated">*62.98%\**</span> <span title="validated upfront with error">*62.94%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.94%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.96%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*959.56 µs\**</span> <span title="prepend">*831.83 µs\**</span> | 3.1910 ms | 489348 | 281173 | 249546 | 3.1274 ms |
| [bincode 2.0.0-rc][bincode] | 310.39 µs | 2.1138 ms | 367413 | 221291 | 206273 | 2.4991 ms |
| [bincode 1.3.3][bincode1] | 600.26 µs | 1.8993 ms | 569975 | 240525 | 232423 | 2.9676 ms |
| [bitcode 0.6.3][bitcode] | 129.63 µs | 1.2816 ms | 327688 | 200947 | 182736 | 741.17 µs |
| [borsh 1.5.1][borsh] | 537.73 µs | 1.8413 ms | 446595 | 234236 | 210008 | 2.4959 ms |
| [capnp 0.19.7][capnp] | 455.48 µs | † | 803896 | 335606 | 280851 | 4.0061 ms |
| [cbor4ii 0.3.3][cbor4ii] | 777.19 µs | 4.6678 ms | 1109831 | 344745 | 274514 | 3.8241 ms |
| [ciborium 0.2.2][ciborium] | 3.6872 ms | 10.414 ms | 1109821 | 344751 | 274526 | 3.8866 ms |
| [databuf 0.5.0][databuf] | 327.40 µs | 1.7395 ms | 356311 | 213062 | 198488 | 2.4076 ms |
| [dlhn 0.1.7][dlhn] | 791.43 µs | 2.6288 ms | 366496 | 220600 | 205683 | 2.5295 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.2421 ms | † | 844168 | 345696 | 294015 | 3.8455 ms |
| [msgpacker 0.4.3][msgpacker] | 992.59 µs | 2.8866 ms | 391251 | 236877 | 220476 | 2.6523 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1246 ms | 3.9535 ms | 449745 | 252432 | 231110 | 2.8288 ms |
| [nanoserde 0.1.37][nanoserde] | 274.24 µs | 1.9240 ms | 567975 | 239930 | 232419 | 2.8829 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 602.03 µs | 1.9916 ms | 356311 | 212976 | 198524 | 2.4614 ms |
| [postcard 1.0.10][postcard] | 451.64 µs | 2.0292 ms | 367489 | 221913 | 207344 | 2.5969 ms |
| [pot 3.0.1][pot] | 2.3892 ms | 6.1148 ms | 599125 | 299158 | 247693 | 3.1825 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.2416 ms\**</span> <span title="populate + encode">*2.9240 ms\**</span> | 3.4109 ms | 596811 | 305319 | 269310 | 3.4727 ms |
| [rkyv 0.8.5][rkyv] | 335.94 µs | <span title="unvalidated">*1.5272 ms\**</span> <span title="validated upfront with error">*2.0472 ms\**</span> | 603776 | 254776 | 220087 | 2.7663 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4094 ms | 3.0624 ms | 424533 | 245214 | 226188 | 2.7298 ms |
| [ron 0.8.1][ron] | 7.3818 ms | 17.480 ms | 1465223 | 434935 | 343338 | 5.8653 ms |
| [savefile 0.17.7][savefile] | 210.11 µs | 1.8538 ms | 566991 | 239361 | 232013 | 2.9047 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3992 ms | 5.3321 ms | 1276014 | 373898 | 293679 | 4.0413 ms |
| [serde_bare 0.5.0][serde_bare] | 740.16 µs | 2.3417 ms | 356311 | 213062 | 198488 | 2.4325 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7122 ms | 4.7556 ms | 1109821 | 344751 | 274526 | 3.9036 ms |
| [serde_json 1.0.128][serde_json] | 3.6817 ms | 6.3959 ms | 1623191 | 466527 | 359623 | 6.0349 ms |
| [serialization 0.2.7][serialization] | 424.19 µs | 1.4663 ms | 356311 | 212514 | 198168 | 2.4136 ms |
| [simd-json 0.13.10][simd-json] | 2.2346 ms | 4.5340 ms | 1623191 | 466527 | 359623 | 6.0637 ms |
| [speedy 0.8.7][speedy] | 274.79 µs | 1.6097 ms | 449595 | 234970 | 210361 | 2.5446 ms |
| [wiring 0.2.2][wiring] | 204.07 µs | 1.8518 ms | 566975 | 247810 | 225259 | 2.9439 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*77.093 ns\**</span> | <span title="validated on-demand with error">*413.07 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4781 ns\**</span> <span title="validated upfront with error">*2.1794 ms\**</span> | <span title="unvalidated">*1.3489 µs\**</span> <span title="validated upfront with error">*2.1996 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2443 ns\**</span> <span title="validated upfront with error">*515.87 µs\**</span> | <span title="unvalidated">*163.59 ns\**</span> <span title="validated upfront with error">*514.57 µs\**</span> | <span title="unvalidated">*784.56 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*13.51%\**</span> <span title="prepend">*15.58%\**</span> | 40.16% | 66.96% | 71.47% | 73.23% | 23.70% |
| [bincode 2.0.0-rc][bincode] | 41.76% | 60.63% | 89.19% | 90.81% | 88.59% | 29.66% |
| [bincode 1.3.3][bincode1] | 21.60% | 67.48% | 57.49% | 83.55% | 78.62% | 24.98% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 24.11% | 69.60% | 73.37% | 85.79% | 87.01% | 29.70% |
| [capnp 0.19.7][capnp] | 28.46% | † | 40.76% | 59.88% | 65.07% | 18.50% |
| [cbor4ii 0.3.3][cbor4ii] | 16.68% | 27.46% | 29.53% | 58.29% | 66.57% | 19.38% |
| [ciborium 0.2.2][ciborium] | 3.52% | 12.31% | 29.53% | 58.29% | 66.56% | 19.07% |
| [databuf 0.5.0][databuf] | 39.59% | 73.68% | 91.97% | 94.31% | 92.06% | 30.78% |
| [dlhn 0.1.7][dlhn] | 16.38% | 48.75% | 89.41% | 91.09% | 88.84% | 29.30% |
| [flatbuffers 24.3.25][flatbuffers] | 4.00% | † | 38.82% | 58.13% | 62.15% | 19.27% |
| [msgpacker 0.4.3][msgpacker] | 13.06% | 44.40% | 83.75% | 84.83% | 82.88% | 27.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.53% | 32.42% | 72.86% | 79.60% | 79.07% | 26.20% |
| [nanoserde 0.1.37][nanoserde] | 47.27% | 66.61% | 57.69% | 83.75% | 78.62% | 25.71% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.53% | 64.35% | 91.97% | 94.35% | 92.05% | 30.11% |
| [postcard 1.0.10][postcard] | 28.70% | 63.16% | 89.17% | 90.55% | 88.13% | 28.54% |
| [pot 3.0.1][pot] | 5.43% | 20.96% | 54.69% | 67.17% | 73.78% | 23.29% |
| [prost 0.13.2][prost] | <span title="encode">*10.44%\**</span> <span title="populate + encode">*4.43%\**</span> | 37.57% | 54.91% | 65.82% | 67.85% | 21.34% |
| [rkyv 0.8.5][rkyv] | 38.59% | <span title="unvalidated">*83.92%\**</span> <span title="validated upfront with error">*62.60%\**</span> | 54.27% | 78.87% | 83.03% | 26.79% |
| [rmp-serde 1.3.0][rmp-serde] | 9.20% | 41.85% | 77.19% | 81.95% | 80.79% | 27.15% |
| [ron 0.8.1][ron] | 1.76% | 7.33% | 22.36% | 46.20% | 53.22% | 12.64% |
| [savefile 0.17.7][savefile] | 61.70% | 69.13% | 57.79% | 83.95% | 78.76% | 25.52% |
| [serde-brief 0.1.0][serde-brief] | 9.26% | 24.04% | 25.68% | 53.74% | 62.22% | 18.34% |
| [serde_bare 0.5.0][serde_bare] | 17.51% | 54.73% | 91.97% | 94.31% | 92.06% | 30.47% |
| [serde_cbor 0.11.2][serde_cbor] | 7.57% | 26.95% | 29.53% | 58.29% | 66.56% | 18.99% |
| [serde_json 1.0.128][serde_json] | 3.52% | 20.04% | 20.19% | 43.07% | 50.81% | 12.28% |
| [serialization 0.2.7][serialization] | 30.56% | 87.40% | 91.97% | 94.56% | 92.21% | 30.71% |
| [simd-json 0.13.10][simd-json] | 5.80% | 28.27% | 20.19% | 43.07% | 50.81% | 12.22% |
| [speedy 0.8.7][speedy] | 47.17% | 79.62% | 72.89% | 85.52% | 86.87% | 29.13% |
| [wiring 0.2.2][wiring] | 63.52% | 69.21% | 57.80% | 81.09% | 81.12% | 25.18% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.61%\**</span> | <span title="validated on-demand with error">*39.60%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.21%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.13%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.7][serialization] | 1.0114 ms | † | † | † | † | † |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.7][serialization] | 100.00% | † | † | † | † | † |

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
[serialization]: https://crates.io/crates/serialization/0.2.7
[simd-json]: https://crates.io/crates/simd-json/0.13.10
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
