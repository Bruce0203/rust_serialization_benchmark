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

## Last updated: 2024-12-16 12:53:52

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
| [bilrost 0.1010.0][bilrost] | <span title="encode">*694.89 µs\**</span> <span title="prepend">*619.08 µs\**</span> | 3.1327 ms | 874632 | 355446 | 311723 | 5.1090 ms |
| [bincode 2.0.0-rc][bincode] | 333.91 µs | 2.5355 ms | 741295 | 303944 | 257153 | 3.9978 ms |
| [bincode 1.3.3][bincode1] | 522.44 µs | 1.9781 ms | 1045784 | 373127 | 311761 | 4.8745 ms |
| [bitcode 0.6.3][bitcode] | 137.86 µs | 1.4767 ms | 703710 | 288826 | 229755 | 2.4383 ms |
| [borsh 1.5.1][borsh] | 545.09 µs | 2.2157 ms | 885780 | 362204 | 286514 | 4.5486 ms |
| [capnp 0.19.7][capnp] | 473.63 µs | † | 1443216 | 513986 | 428649 | 6.8268 ms |
| [cbor4ii 0.3.3][cbor4ii] | 597.46 µs | 4.9927 ms | 1407835 | 403440 | 324081 | 4.8581 ms |
| [ciborium 0.2.2][ciborium] | 3.9868 ms | 12.167 ms | 1407835 | 403440 | 324081 | 4.8980 ms |
| [databuf 0.5.0][databuf] | 264.37 µs | 2.0635 ms | 765778 | 311715 | 264630 | 4.1864 ms |
| [dlhn 0.1.7][dlhn] | 744.96 µs | 2.5771 ms | 724953 | 301446 | 253629 | 3.8187 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0158 ms | † | 1276368 | 468539 | 388832 | 5.1631 ms |
| [msgpacker 0.4.3][msgpacker] | 1.3888 ms | 2.5830 ms | 764996 | 315291 | 264898 | 4.1907 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4815 ms | 4.2111 ms | 818669 | 332556 | 285514 | 4.6659 ms |
| [nanoserde 0.1.37][nanoserde] | 293.76 µs | 2.1110 ms | 1045784 | 373127 | 311761 | 4.8548 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 644.78 µs | 2.2710 ms | 765778 | 311743 | 264518 | 3.9706 ms |
| [postcard 1.0.10][postcard] | 422.36 µs | 2.1993 ms | 724953 | 302399 | 253747 | 3.6634 ms |
| [pot 3.0.1][pot] | 2.2532 ms | 6.4569 ms | 971922 | 372513 | 304122 | 4.9515 ms |
| [prost 0.13.2][prost] | <span title="encode">*932.06 µs\**</span> <span title="populate + encode">*2.4748 ms\**</span> | 3.3298 ms | 884628 | 363130 | 315494 | 4.8624 ms |
| [rkyv 0.8.5][rkyv] | 238.55 µs | <span title="unvalidated">*1.5933 ms\**</span> <span title="validated upfront with error">*2.2077 ms\**</span> | 1011488 | 393526 | 326517 | 5.1252 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3848 ms | 3.1796 ms | 784997 | 325384 | 278219 | 4.2268 ms |
| [ron 0.8.1][ron] | 11.393 ms | 17.134 ms | 1607459 | 449158 | 349713 | 6.2058 ms |
| [savefile 0.17.7][savefile] | 191.67 µs | 2.2240 ms | 1045800 | 373140 | 311777 | 4.8613 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5356 ms | 4.7199 ms | 1584946 | 413733 | 341439 | 4.9405 ms |
| [serde_bare 0.5.0][serde_bare] | 681.63 µs | 2.1171 ms | 765778 | 311715 | 264630 | 4.1493 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0415 ms | 4.8857 ms | 1407835 | 403440 | 324081 | 5.1379 ms |
| [serde_json 1.0.128][serde_json] | 4.1356 ms | 5.4153 ms | 1827461 | 470560 | 361090 | 6.1037 ms |
| [serialization 0.2.7][serialization] | 739.48 µs | 1.8186 ms | 765778 | 305641 | 259060 | 4.0796 ms |
| [simd-json 0.13.10][simd-json] | 2.1772 ms | 4.6849 ms | 1827461 | 470560 | 361090 | 5.7164 ms |
| [speedy 0.8.7][speedy] | 195.28 µs | 1.8000 ms | 885780 | 362204 | 286514 | 4.1994 ms |
| [wiring 0.2.2][wiring] | 193.91 µs | 1.9730 ms | 1045784 | 337930 | 276188 | 3.9582 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*75.285 ns\**</span> | <span title="validated on-demand with error">*168.24 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4737 ns\**</span> <span title="validated upfront with error">*2.0481 ms\**</span> | <span title="unvalidated">*50.550 µs\**</span> <span title="validated upfront with error">*2.0373 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2378 ns\**</span> <span title="validated upfront with error">*613.74 µs\**</span> | <span title="unvalidated">*10.524 µs\**</span> <span title="validated upfront with error">*621.07 µs\**</span> | <span title="unvalidated">*7.3859 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*19.84%\**</span> <span title="prepend">*22.27%\**</span> | 47.14% | 80.46% | 81.26% | 73.70% | 47.73% |
| [bincode 2.0.0-rc][bincode] | 41.29% | 58.24% | 94.93% | 95.03% | 89.35% | 60.99% |
| [bincode 1.3.3][bincode1] | 26.39% | 74.65% | 67.29% | 77.41% | 73.70% | 50.02% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.29% | 66.65% | 79.45% | 79.74% | 80.19% | 53.61% |
| [capnp 0.19.7][capnp] | 29.11% | † | 48.76% | 56.19% | 53.60% | 35.72% |
| [cbor4ii 0.3.3][cbor4ii] | 23.07% | 29.58% | 49.99% | 71.59% | 70.89% | 50.19% |
| [ciborium 0.2.2][ciborium] | 3.46% | 12.14% | 49.99% | 71.59% | 70.89% | 49.78% |
| [databuf 0.5.0][databuf] | 52.15% | 71.56% | 91.89% | 92.66% | 86.82% | 58.24% |
| [dlhn 0.1.7][dlhn] | 18.51% | 57.30% | 97.07% | 95.81% | 90.59% | 63.85% |
| [flatbuffers 24.3.25][flatbuffers] | 13.57% | † | 55.13% | 61.64% | 59.09% | 47.23% |
| [msgpacker 0.4.3][msgpacker] | 9.93% | 57.17% | 91.99% | 91.61% | 86.73% | 58.18% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 35.07% | 85.96% | 86.85% | 80.47% | 52.26% |
| [nanoserde 0.1.37][nanoserde] | 46.93% | 69.95% | 67.29% | 77.41% | 73.70% | 50.22% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.38% | 65.02% | 91.89% | 92.65% | 86.86% | 61.41% |
| [postcard 1.0.10][postcard] | 32.64% | 67.14% | 97.07% | 95.51% | 90.54% | 66.56% |
| [pot 3.0.1][pot] | 6.12% | 22.87% | 72.40% | 77.53% | 75.55% | 49.24% |
| [prost 0.13.2][prost] | <span title="encode">*14.79%\**</span> <span title="populate + encode">*5.57%\**</span> | 44.35% | 79.55% | 79.54% | 72.82% | 50.15% |
| [rkyv 0.8.5][rkyv] | 57.79% | <span title="unvalidated">*92.68%\**</span> <span title="validated upfront with error">*66.89%\**</span> | 69.57% | 73.39% | 70.37% | 47.57% |
| [rmp-serde 1.3.0][rmp-serde] | 9.96% | 46.44% | 89.64% | 88.76% | 82.58% | 57.69% |
| [ron 0.8.1][ron] | 1.21% | 8.62% | 43.78% | 64.30% | 65.70% | 39.29% |
| [savefile 0.17.7][savefile] | 71.93% | 66.40% | 67.29% | 77.40% | 73.69% | 50.16% |
| [serde-brief 0.1.0][serde-brief] | 8.98% | 31.29% | 44.40% | 69.81% | 67.29% | 49.35% |
| [serde_bare 0.5.0][serde_bare] | 20.23% | 69.75% | 91.89% | 92.66% | 86.82% | 58.76% |
| [serde_cbor 0.11.2][serde_cbor] | 6.75% | 30.22% | 49.99% | 71.59% | 70.89% | 47.46% |
| [serde_json 1.0.128][serde_json] | 3.33% | 27.27% | 38.51% | 61.38% | 63.63% | 39.95% |
| [serialization 0.2.7][serialization] | 18.64% | 81.20% | 91.89% | 94.50% | 88.69% | 59.77% |
| [simd-json 0.13.10][simd-json] | 6.33% | 31.52% | 38.51% | 61.38% | 63.63% | 42.65% |
| [speedy 0.8.7][speedy] | 70.60% | 82.04% | 79.45% | 79.74% | 80.19% | 58.06% |
| [wiring 0.2.2][wiring] | 71.09% | 74.85% | 67.29% | 85.47% | 83.19% | 61.60% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="validated on-demand with error">*6.26%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.82%\**</span> <span title="validated upfront with error">*0.52%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.69%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6962 ms\**</span> <span title="prepend">*8.7444 ms\**</span> | 8.8877 ms | 8625005 | 6443961 | 6231572 | 71.161 ms |
| [bincode 2.0.0-rc][bincode] | 2.3958 ms | 1.4023 ms | 6000005 | 5378497 | 5345897 | 7.9198 ms |
| [bincode 1.3.3][bincode1] | 5.1593 ms | 4.7183 ms | 6000008 | 5378500 | 5345890 | 7.8496 ms |
| [bitcode 0.6.3][bitcode] | 1.4183 ms | 793.55 µs | 6000006 | 5182295 | 4923880 | 12.671 ms |
| [borsh 1.5.1][borsh] | 6.1277 ms | 4.5305 ms | 6000004 | 5378496 | 5345889 | 7.7682 ms |
| [capnp 0.19.7][capnp] | 5.4638 ms | † | 14000088 | 7130367 | 6051062 | 78.294 ms |
| [cbor4ii 0.3.3][cbor4ii] | 10.229 ms | 51.150 ms | 13125016 | 7524114 | 6757967 | 88.731 ms |
| [ciborium 0.2.2][ciborium] | 67.926 ms | 115.08 ms | 13122324 | 7524660 | 6759658 | 89.027 ms |
| [databuf 0.5.0][databuf] | 2.3968 ms | 5.3535 ms | 6000003 | 5378495 | 5345900 | 7.7788 ms |
| [dlhn 0.1.7][dlhn] | 6.1724 ms | 7.0382 ms | 6000003 | 5378495 | 5345900 | 7.8582 ms |
| [flatbuffers 24.3.25][flatbuffers] | 860.02 µs | † | 6000024 | 5378434 | 5345910 | 7.9321 ms |
| [msgpacker 0.4.3][msgpacker] | 18.427 ms | 5.2180 ms | 7500005 | 6058442 | 6014337 | 9.5330 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 124.20 ms | 32.899 ms | 8125037 | 6493484 | 6386940 | 67.741 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2811 ms | 1.0982 ms | 6000008 | 5378500 | 5345890 | 7.4578 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.6349 ms | 4.0119 ms | 6000004 | 5378496 | 5345889 | 7.6685 ms |
| [postcard 1.0.10][postcard] | 476.21 µs | 1.7379 ms | 6000003 | 5378495 | 5345900 | 7.7798 ms |
| [pot 3.0.1][pot] | 36.418 ms | 72.142 ms | 10122342 | 6814618 | 6852251 | 79.440 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7194 ms\**</span> <span title="populate + encode">*8.3164 ms\**</span> | 13.601 ms | 8750000 | 6665735 | 6421871 | 69.907 ms |
| [rkyv 0.8.5][rkyv] | 238.38 µs | <span title="unvalidated">*149.22 µs\**</span> <span title="validated upfront with error">*148.19 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.6701 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.546 ms | 17.960 ms | 8125006 | 6494876 | 6391037 | 67.087 ms |
| [ron 0.8.1][ron] | 174.65 ms | 235.01 ms | 22192885 | 8970395 | 8138755 | 146.69 ms |
| [savefile 0.17.7][savefile] | 241.18 µs | 238.94 µs | 6000024 | 5378513 | 5345893 | 7.5626 ms |
| [serde-brief 0.1.0][serde-brief] | 22.075 ms | 42.241 ms | 15750015 | 8024540 | 6816643 | 90.972 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1975 ms | 4.7385 ms | 6000003 | 5378495 | 5345900 | 7.5107 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.399 ms | 47.045 ms | 13122324 | 7524660 | 6759658 | 88.310 ms |
| [serde_json 1.0.128][serde_json] | 87.875 ms | 86.895 ms | 26192883 | 9566084 | 8586741 | 151.74 ms |
| [serialization 0.2.7][serialization] | 344.42 µs | 199.18 µs | 6000003 | 5378495 | 5345900 | 8.0147 ms |
| [simd-json 0.13.10][simd-json] | 53.522 ms | 76.302 ms | 26192883 | 9566084 | 8586741 | 151.76 ms |
| [speedy 0.8.7][speedy] | 238.22 µs | 239.12 µs | 6000004 | 5378496 | 5345889 | 7.7850 ms |
| [wiring 0.2.2][wiring] | 149.21 µs | 334.78 µs | 6000008 | 5378952 | 5345894 | 7.8759 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*105.06 ns\**</span> | <span title="validated on-demand with error">*2.1821 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4746 ns\**</span> <span title="validated upfront with error">*39.842 ns\**</span> | <span title="unvalidated">*54.122 µs\**</span> <span title="validated upfront with error">*78.315 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*4.9557 ns\**</span> | <span title="unvalidated">*48.389 µs\**</span> <span title="validated upfront with error">*38.877 µs\**</span> | <span title="unvalidated">*77.643 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.23%\**</span> <span title="prepend">*1.71%\**</span> | 1.67% | 69.57% | 80.42% | 79.02% | 10.48% |
| [bincode 2.0.0-rc][bincode] | 6.23% | 10.57% | 100.00% | 96.35% | 92.11% | 94.17% |
| [bincode 1.3.3][bincode1] | 2.89% | 3.14% | 100.00% | 96.35% | 92.11% | 95.01% |
| [bitcode 0.6.3][bitcode] | 10.52% | 18.67% | 100.00% | 100.00% | 100.00% | 58.86% |
| [borsh 1.5.1][borsh] | 2.44% | 3.27% | 100.00% | 96.35% | 92.11% | 96.00% |
| [capnp 0.19.7][capnp] | 2.73% | † | 42.86% | 72.68% | 81.37% | 9.53% |
| [cbor4ii 0.3.3][cbor4ii] | 1.46% | 0.29% | 45.71% | 68.88% | 72.86% | 8.40% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.84% | 8.38% |
| [databuf 0.5.0][databuf] | 6.23% | 2.77% | 100.00% | 96.35% | 92.11% | 95.87% |
| [dlhn 0.1.7][dlhn] | 2.42% | 2.11% | 100.00% | 96.35% | 92.11% | 94.90% |
| [flatbuffers 24.3.25][flatbuffers] | 17.35% | † | 100.00% | 96.35% | 92.11% | 94.02% |
| [msgpacker 0.4.3][msgpacker] | 0.81% | 2.84% | 80.00% | 85.54% | 81.87% | 78.23% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.45% | 73.85% | 79.81% | 77.09% | 11.01% |
| [nanoserde 0.1.37][nanoserde] | 11.65% | 13.49% | 100.00% | 96.35% | 92.11% | 100.00% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.22% | 3.69% | 100.00% | 96.35% | 92.11% | 97.25% |
| [postcard 1.0.10][postcard] | 31.33% | 8.53% | 100.00% | 96.35% | 92.11% | 95.86% |
| [pot 3.0.1][pot] | 0.41% | 0.21% | 59.27% | 76.05% | 71.86% | 9.39% |
| [prost 0.13.2][prost] | <span title="encode">*1.93%\**</span> <span title="populate + encode">*1.79%\**</span> | 1.09% | 68.57% | 77.75% | 76.67% | 10.67% |
| [rkyv 0.8.5][rkyv] | 62.59% | <span title="unvalidated">*99.31%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 97.23% |
| [rmp-serde 1.3.0][rmp-serde] | 0.96% | 0.83% | 73.85% | 79.79% | 77.04% | 11.12% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 5.08% |
| [savefile 0.17.7][savefile] | 61.87% | 62.02% | 100.00% | 96.35% | 92.11% | 98.61% |
| [serde-brief 0.1.0][serde-brief] | 0.68% | 0.35% | 38.10% | 64.58% | 72.23% | 8.20% |
| [serde_bare 0.5.0][serde_bare] | 2.41% | 3.13% | 100.00% | 96.35% | 92.11% | 99.30% |
| [serde_cbor 0.11.2][serde_cbor] | 0.43% | 0.31% | 45.72% | 68.87% | 72.84% | 8.45% |
| [serde_json 1.0.128][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.91% |
| [serialization 0.2.7][serialization] | 43.32% | 74.40% | 100.00% | 96.35% | 92.11% | 93.05% |
| [simd-json 0.13.10][simd-json] | 0.28% | 0.19% | 22.91% | 54.17% | 57.34% | 4.91% |
| [speedy 0.8.7][speedy] | 62.64% | 61.97% | 100.00% | 96.35% | 92.11% | 95.80% |
| [wiring 0.2.2][wiring] | 100.00% | 44.26% | 100.00% | 96.34% | 92.11% | 94.69% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.78%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*3.11%\**</span> | <span title="unvalidated">*71.83%\**</span> <span title="validated upfront with error">*49.64%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.96%\**</span> | <span title="unvalidated">*80.34%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*942.71 µs\**</span> <span title="prepend">*827.60 µs\**</span> | 3.2212 ms | 489348 | 281173 | 249546 | 3.0883 ms |
| [bincode 2.0.0-rc][bincode] | 315.15 µs | 2.0941 ms | 367413 | 221291 | 206273 | 2.4856 ms |
| [bincode 1.3.3][bincode1] | 594.76 µs | 1.8712 ms | 569975 | 240525 | 232423 | 3.2667 ms |
| [bitcode 0.6.3][bitcode] | 128.92 µs | 1.2768 ms | 327688 | 200947 | 182736 | 734.68 µs |
| [borsh 1.5.1][borsh] | 551.99 µs | 1.8550 ms | 446595 | 234236 | 210008 | 2.4871 ms |
| [capnp 0.19.7][capnp] | 477.38 µs | † | 803896 | 335606 | 280851 | 3.8976 ms |
| [cbor4ii 0.3.3][cbor4ii] | 796.11 µs | 4.8715 ms | 1109831 | 344745 | 274514 | 3.8872 ms |
| [ciborium 0.2.2][ciborium] | 3.7999 ms | 10.217 ms | 1109821 | 344751 | 274526 | 3.8345 ms |
| [databuf 0.5.0][databuf] | 328.91 µs | 1.7572 ms | 356311 | 213062 | 198488 | 2.3765 ms |
| [dlhn 0.1.7][dlhn] | 792.03 µs | 2.6248 ms | 366496 | 220600 | 205683 | 2.4792 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.3454 ms | † | 844168 | 345696 | 294015 | 3.8213 ms |
| [msgpacker 0.4.3][msgpacker] | 981.84 µs | 2.9080 ms | 391251 | 236877 | 220476 | 2.6389 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2842 ms | 4.0027 ms | 449745 | 252432 | 231110 | 2.7597 ms |
| [nanoserde 0.1.37][nanoserde] | 281.40 µs | 1.9006 ms | 567975 | 239930 | 232419 | 2.8693 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 609.32 µs | 1.9911 ms | 356311 | 212976 | 198524 | 2.4085 ms |
| [postcard 1.0.10][postcard] | 440.85 µs | 2.0338 ms | 367489 | 221913 | 207344 | 2.4937 ms |
| [pot 3.0.1][pot] | 2.3817 ms | 6.0660 ms | 599125 | 299158 | 247693 | 3.1538 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.0959 ms\**</span> <span title="populate + encode">*2.7901 ms\**</span> | 3.4761 ms | 596811 | 305319 | 269310 | 3.4356 ms |
| [rkyv 0.8.5][rkyv] | 348.71 µs | <span title="unvalidated">*1.5168 ms\**</span> <span title="validated upfront with error">*2.0350 ms\**</span> | 603776 | 254776 | 220087 | 2.7234 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4260 ms | 3.0368 ms | 424533 | 245214 | 226188 | 2.6876 ms |
| [ron 0.8.1][ron] | 7.3583 ms | 17.306 ms | 1465223 | 434935 | 343338 | 5.8387 ms |
| [savefile 0.17.7][savefile] | 210.13 µs | 1.8438 ms | 566991 | 239361 | 232013 | 2.8924 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3290 ms | 5.3529 ms | 1276014 | 373898 | 293679 | 4.0676 ms |
| [serde_bare 0.5.0][serde_bare] | 747.17 µs | 2.3698 ms | 356311 | 213062 | 198488 | 2.4062 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8382 ms | 4.7634 ms | 1109821 | 344751 | 274526 | 3.8481 ms |
| [serde_json 1.0.128][serde_json] | 3.8731 ms | 6.2824 ms | 1623191 | 466527 | 359623 | 6.0622 ms |
| [serialization 0.2.7][serialization] | 654.82 µs | 1.7822 ms | 356311 | 211955 | 195923 | 2.3815 ms |
| [simd-json 0.13.10][simd-json] | 2.3354 ms | 4.5518 ms | 1623191 | 466527 | 359623 | 6.0427 ms |
| [speedy 0.8.7][speedy] | 270.67 µs | 1.6097 ms | 449595 | 234970 | 210361 | 2.5066 ms |
| [wiring 0.2.2][wiring] | 205.51 µs | 1.8441 ms | 566975 | 247810 | 225259 | 2.9060 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*75.299 ns\**</span> | <span title="validated on-demand with error">*409.35 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4731 ns\**</span> <span title="validated upfront with error">*2.2337 ms\**</span> | <span title="unvalidated">*1.3655 µs\**</span> <span title="validated upfront with error">*2.1930 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2373 ns\**</span> <span title="validated upfront with error">*527.63 µs\**</span> | <span title="unvalidated">*163.10 ns\**</span> <span title="validated upfront with error">*527.58 µs\**</span> | <span title="unvalidated">*740.22 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*13.68%\**</span> <span title="prepend">*15.58%\**</span> | 39.64% | 66.96% | 71.47% | 73.23% | 23.79% |
| [bincode 2.0.0-rc][bincode] | 40.91% | 60.97% | 89.19% | 90.81% | 88.59% | 29.56% |
| [bincode 1.3.3][bincode1] | 21.68% | 68.23% | 57.49% | 83.55% | 78.62% | 22.49% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 23.36% | 68.83% | 73.37% | 85.79% | 87.01% | 29.54% |
| [capnp 0.19.7][capnp] | 27.01% | † | 40.76% | 59.88% | 65.07% | 18.85% |
| [cbor4ii 0.3.3][cbor4ii] | 16.19% | 26.21% | 29.53% | 58.29% | 66.57% | 18.90% |
| [ciborium 0.2.2][ciborium] | 3.39% | 12.50% | 29.53% | 58.29% | 66.56% | 19.16% |
| [databuf 0.5.0][databuf] | 39.20% | 72.66% | 91.97% | 94.31% | 92.06% | 30.91% |
| [dlhn 0.1.7][dlhn] | 16.28% | 48.64% | 89.41% | 91.09% | 88.84% | 29.63% |
| [flatbuffers 24.3.25][flatbuffers] | 3.85% | † | 38.82% | 58.13% | 62.15% | 19.23% |
| [msgpacker 0.4.3][msgpacker] | 13.13% | 43.91% | 83.75% | 84.83% | 82.88% | 27.84% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.44% | 31.90% | 72.86% | 79.60% | 79.07% | 26.62% |
| [nanoserde 0.1.37][nanoserde] | 45.81% | 67.18% | 57.69% | 83.75% | 78.62% | 25.60% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.16% | 64.13% | 91.97% | 94.35% | 92.05% | 30.50% |
| [postcard 1.0.10][postcard] | 29.24% | 62.78% | 89.17% | 90.55% | 88.13% | 29.46% |
| [pot 3.0.1][pot] | 5.41% | 21.05% | 54.69% | 67.17% | 73.78% | 23.30% |
| [prost 0.13.2][prost] | <span title="encode">*11.76%\**</span> <span title="populate + encode">*4.62%\**</span> | 36.73% | 54.91% | 65.82% | 67.85% | 21.38% |
| [rkyv 0.8.5][rkyv] | 36.97% | <span title="unvalidated">*84.18%\**</span> <span title="validated upfront with error">*62.74%\**</span> | 54.27% | 78.87% | 83.03% | 26.98% |
| [rmp-serde 1.3.0][rmp-serde] | 9.04% | 42.04% | 77.19% | 81.95% | 80.79% | 27.34% |
| [ron 0.8.1][ron] | 1.75% | 7.38% | 22.36% | 46.20% | 53.22% | 12.58% |
| [savefile 0.17.7][savefile] | 61.35% | 69.25% | 57.79% | 83.95% | 78.76% | 25.40% |
| [serde-brief 0.1.0][serde-brief] | 9.70% | 23.85% | 25.68% | 53.74% | 62.22% | 18.06% |
| [serde_bare 0.5.0][serde_bare] | 17.25% | 53.88% | 91.97% | 94.31% | 92.06% | 30.53% |
| [serde_cbor 0.11.2][serde_cbor] | 7.01% | 26.80% | 29.53% | 58.29% | 66.56% | 19.09% |
| [serde_json 1.0.128][serde_json] | 3.33% | 20.32% | 20.19% | 43.07% | 50.81% | 12.12% |
| [serialization 0.2.7][serialization] | 19.69% | 71.64% | 91.97% | 94.81% | 93.27% | 30.85% |
| [simd-json 0.13.10][simd-json] | 5.52% | 28.05% | 20.19% | 43.07% | 50.81% | 12.16% |
| [speedy 0.8.7][speedy] | 47.63% | 79.32% | 72.89% | 85.52% | 86.87% | 29.31% |
| [wiring 0.2.2][wiring] | 62.73% | 69.24% | 57.80% | 81.09% | 81.12% | 25.28% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="validated on-demand with error">*39.84%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.94%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.7][serialization] | 2.7406 ms | † | † | † | † | † |

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
