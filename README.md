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

## Last updated: 2024-12-17 19:48:20

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.85.0-nightly (6d9f6ae36 2024-12-16)
binary: rustc
commit-hash: 6d9f6ae36ae1299d6126ba40c15191f7aa3b79d8
commit-date: 2024-12-16
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
BogoMIPS:                           4890.87
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
| [bilrost 0.1010.0][bilrost] | <span title="encode">*695.65 µs\**</span> <span title="prepend">*620.47 µs\**</span> | 3.1313 ms | 874632 | 355446 | 311723 | 5.0876 ms |
| [bincode 2.0.0-rc][bincode] | 323.16 µs | 2.5096 ms | 741295 | 303944 | 257153 | 3.9962 ms |
| [bincode 1.3.3][bincode1] | 519.87 µs | 1.9766 ms | 1045784 | 373127 | 311761 | 4.8666 ms |
| [bitcode 0.6.3][bitcode] | 143.49 µs | 1.4645 ms | 703710 | 288826 | 229755 | 2.4214 ms |
| [borsh 1.5.1][borsh] | 540.46 µs | 2.1914 ms | 885780 | 362204 | 286514 | 4.4924 ms |
| [capnp 0.19.7][capnp] | 516.87 µs | † | 1443216 | 513986 | 428649 | 6.7727 ms |
| [cbor4ii 0.3.3][cbor4ii] | 594.13 µs | 4.9541 ms | 1407835 | 403440 | 324081 | 4.8452 ms |
| [ciborium 0.2.2][ciborium] | 3.9641 ms | 11.813 ms | 1407835 | 403440 | 324081 | 4.9283 ms |
| [databuf 0.5.0][databuf] | 259.05 µs | 2.0736 ms | 765778 | 311715 | 264630 | 3.8869 ms |
| [dlhn 0.1.7][dlhn] | 730.55 µs | 2.5361 ms | 724953 | 301446 | 253629 | 3.5750 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0893 ms | † | 1276368 | 468539 | 388832 | 5.1882 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2783 ms | 2.5762 ms | 764996 | 315291 | 264898 | 3.9133 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5663 ms | 4.0843 ms | 818669 | 332556 | 285514 | 4.3550 ms |
| [nanoserde 0.1.37][nanoserde] | 338.53 µs | 2.1248 ms | 1045784 | 373127 | 311761 | 4.8223 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 644.46 µs | 2.2224 ms | 765778 | 311743 | 264518 | 3.8872 ms |
| [postcard 1.0.10][postcard] | 421.97 µs | 2.1863 ms | 724953 | 302399 | 253747 | 3.8202 ms |
| [pot 3.0.1][pot] | 2.2952 ms | 6.4751 ms | 971922 | 372513 | 304122 | 4.6112 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.0245 ms\**</span> <span title="populate + encode">*2.4721 ms\**</span> | 3.3460 ms | 884628 | 363130 | 315494 | 5.1558 ms |
| [rkyv 0.8.5][rkyv] | 408.02 µs | <span title="unvalidated">*1.5898 ms\**</span> <span title="validated upfront with error">*2.1892 ms\**</span> | 1011488 | 393526 | 326517 | 5.3458 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3883 ms | 3.1757 ms | 784997 | 325384 | 278219 | 4.3952 ms |
| [ron 0.8.1][ron] | 11.396 ms | 15.666 ms | 1607459 | 449158 | 349713 | 6.0268 ms |
| [savefile 0.17.7][savefile] | 190.74 µs | 2.2341 ms | 1045800 | 373140 | 311777 | 4.8052 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5525 ms | 5.1549 ms | 1584946 | 413733 | 341439 | 4.9489 ms |
| [serde_bare 0.5.0][serde_bare] | 675.17 µs | 2.1051 ms | 765778 | 311715 | 264630 | 4.1380 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.1165 ms | 4.8678 ms | 1407835 | 403440 | 324081 | 5.0602 ms |
| [serde_json 1.0.128][serde_json] | 3.8475 ms | 5.4439 ms | 1827461 | 470560 | 361090 | 5.9407 ms |
| [serialization 0.2.8][serialization] | 717.51 µs | 1.8040 ms | 765778 | 305641 | 259060 | 4.0222 ms |
| [speedy 0.8.7][speedy] | 208.32 µs | 1.7930 ms | 885780 | 362204 | 286514 | 4.6085 ms |
| [wiring 0.2.2][wiring] | 192.23 µs | 1.9906 ms | 1045784 | 337930 | 276188 | 3.9316 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*73.059 ns\**</span> | <span title="validated on-demand with error">*167.61 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4732 ns\**</span> <span title="validated upfront with error">*1.9832 ms\**</span> | <span title="unvalidated">*49.145 µs\**</span> <span title="validated upfront with error">*2.1004 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*604.21 µs\**</span> | <span title="unvalidated">*10.508 µs\**</span> <span title="validated upfront with error">*617.14 µs\**</span> | <span title="unvalidated">*7.5627 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*20.63%\**</span> <span title="prepend">*23.13%\**</span> | 46.77% | 80.46% | 81.26% | 73.70% | 47.59% |
| [bincode 2.0.0-rc][bincode] | 44.40% | 58.36% | 94.93% | 95.03% | 89.35% | 60.59% |
| [bincode 1.3.3][bincode1] | 27.60% | 74.09% | 67.29% | 77.41% | 73.70% | 49.76% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 26.55% | 66.83% | 79.45% | 79.74% | 80.19% | 53.90% |
| [capnp 0.19.7][capnp] | 27.76% | † | 48.76% | 56.19% | 53.60% | 35.75% |
| [cbor4ii 0.3.3][cbor4ii] | 24.15% | 29.56% | 49.99% | 71.59% | 70.89% | 49.98% |
| [ciborium 0.2.2][ciborium] | 3.62% | 12.40% | 49.99% | 71.59% | 70.89% | 49.13% |
| [databuf 0.5.0][databuf] | 55.39% | 70.63% | 91.89% | 92.66% | 86.82% | 62.30% |
| [dlhn 0.1.7][dlhn] | 19.64% | 57.75% | 97.07% | 95.81% | 90.59% | 67.73% |
| [flatbuffers 24.3.25][flatbuffers] | 13.17% | † | 55.13% | 61.64% | 59.09% | 46.67% |
| [msgpacker 0.4.3][msgpacker] | 11.23% | 56.85% | 91.99% | 91.61% | 86.73% | 61.88% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.58% | 35.86% | 85.96% | 86.85% | 80.47% | 55.60% |
| [nanoserde 0.1.37][nanoserde] | 42.39% | 68.92% | 67.29% | 77.41% | 73.70% | 50.21% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.27% | 65.90% | 91.89% | 92.65% | 86.86% | 62.29% |
| [postcard 1.0.10][postcard] | 34.00% | 66.99% | 97.07% | 95.51% | 90.54% | 63.38% |
| [pot 3.0.1][pot] | 6.25% | 22.62% | 72.40% | 77.53% | 75.55% | 52.51% |
| [prost 0.13.2][prost] | <span title="encode">*14.01%\**</span> <span title="populate + encode">*5.80%\**</span> | 43.77% | 79.55% | 79.54% | 72.82% | 46.96% |
| [rkyv 0.8.5][rkyv] | 35.17% | <span title="unvalidated">*92.12%\**</span> <span title="validated upfront with error">*66.90%\**</span> | 69.57% | 73.39% | 70.37% | 45.30% |
| [rmp-serde 1.3.0][rmp-serde] | 10.34% | 46.12% | 89.64% | 88.76% | 82.58% | 55.09% |
| [ron 0.8.1][ron] | 1.26% | 9.35% | 43.78% | 64.30% | 65.70% | 40.18% |
| [savefile 0.17.7][savefile] | 75.23% | 65.55% | 67.29% | 77.40% | 73.69% | 50.39% |
| [serde-brief 0.1.0][serde-brief] | 9.24% | 28.41% | 44.40% | 69.81% | 67.29% | 48.93% |
| [serde_bare 0.5.0][serde_bare] | 21.25% | 69.57% | 91.89% | 92.66% | 86.82% | 58.52% |
| [serde_cbor 0.11.2][serde_cbor] | 6.78% | 30.09% | 49.99% | 71.59% | 70.89% | 47.85% |
| [serde_json 1.0.128][serde_json] | 3.73% | 26.90% | 38.51% | 61.38% | 63.63% | 40.76% |
| [serialization 0.2.8][serialization] | 20.00% | 81.18% | 91.89% | 94.50% | 88.69% | 60.20% |
| [speedy 0.8.7][speedy] | 68.88% | 81.68% | 79.45% | 79.74% | 80.19% | 52.54% |
| [wiring 0.2.2][wiring] | 74.64% | 73.57% | 67.29% | 85.47% | 83.19% | 61.59% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*6.27%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.38%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.70%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.7010 ms\**</span> <span title="prepend">*8.8156 ms\**</span> | 9.1024 ms | 8625005 | 6443961 | 6231572 | 70.717 ms |
| [bincode 2.0.0-rc][bincode] | 2.3986 ms | 1.0559 ms | 6000005 | 5378497 | 5345897 | 8.0322 ms |
| [bincode 1.3.3][bincode1] | 5.0948 ms | 4.7515 ms | 6000008 | 5378500 | 5345890 | 7.4657 ms |
| [bitcode 0.6.3][bitcode] | 1.4177 ms | 793.54 µs | 6000006 | 5182295 | 4923880 | 13.032 ms |
| [borsh 1.5.1][borsh] | 6.1042 ms | 4.1301 ms | 6000004 | 5378496 | 5345889 | 7.7256 ms |
| [capnp 0.19.7][capnp] | 5.6740 ms | † | 14000088 | 7130367 | 6051062 | 79.294 ms |
| [cbor4ii 0.3.3][cbor4ii] | 10.244 ms | 50.462 ms | 13125016 | 7524114 | 6757967 | 92.419 ms |
| [ciborium 0.2.2][ciborium] | 67.685 ms | 116.64 ms | 13122324 | 7524660 | 6759658 | 92.525 ms |
| [databuf 0.5.0][databuf] | 2.3958 ms | 5.3111 ms | 6000003 | 5378495 | 5345900 | 7.5693 ms |
| [dlhn 0.1.7][dlhn] | 6.3961 ms | 6.7004 ms | 6000003 | 5378495 | 5345900 | 7.5583 ms |
| [flatbuffers 24.3.25][flatbuffers] | 859.69 µs | † | 6000024 | 5378434 | 5345910 | 7.6631 ms |
| [msgpacker 0.4.3][msgpacker] | 18.688 ms | 5.1605 ms | 7500005 | 6058442 | 6014337 | 10.943 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.01 ms | 32.318 ms | 8125037 | 6493484 | 6386940 | 73.999 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3342 ms | 1.1030 ms | 6000008 | 5378500 | 5345890 | 7.8290 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0782 ms | 3.9889 ms | 6000004 | 5378496 | 5345889 | 7.5043 ms |
| [postcard 1.0.10][postcard] | 479.85 µs | 1.3326 ms | 6000003 | 5378495 | 5345900 | 7.7869 ms |
| [pot 3.0.1][pot] | 38.301 ms | 74.522 ms | 10122342 | 6814618 | 6852251 | 81.121 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7323 ms\**</span> <span title="populate + encode">*8.3389 ms\**</span> | 13.557 ms | 8750000 | 6665735 | 6421871 | 70.148 ms |
| [rkyv 0.8.5][rkyv] | 238.65 µs | <span title="unvalidated">*147.60 µs\**</span> <span title="validated upfront with error">*148.09 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.4331 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.301 ms | 17.285 ms | 8125006 | 6494876 | 6391037 | 66.972 ms |
| [ron 0.8.1][ron] | 170.38 ms | 243.79 ms | 22192885 | 8970395 | 8138755 | 150.30 ms |
| [savefile 0.17.7][savefile] | 238.79 µs | 237.61 µs | 6000024 | 5378513 | 5345893 | 7.7198 ms |
| [serde-brief 0.1.0][serde-brief] | 22.467 ms | 37.132 ms | 15750015 | 8024540 | 6816643 | 93.272 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1886 ms | 4.7461 ms | 6000003 | 5378495 | 5345900 | 7.8019 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.064 ms | 46.178 ms | 13122324 | 7524660 | 6759658 | 95.846 ms |
| [serde_json 1.0.128][serde_json] | 87.868 ms | 85.416 ms | 26192883 | 9566084 | 8586741 | 155.12 ms |
| [serialization 0.2.8][serialization] | 343.34 µs | 197.35 µs | 6000003 | 5378495 | 5345900 | 7.7210 ms |
| [speedy 0.8.7][speedy] | 238.60 µs | 238.17 µs | 6000004 | 5378496 | 5345889 | 7.5263 ms |
| [wiring 0.2.2][wiring] | 198.73 µs | 352.43 µs | 6000008 | 5378952 | 5345894 | 7.6215 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*105.26 ns\**</span> | <span title="validated on-demand with error">*2.1499 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.5081 ns\**</span> <span title="validated upfront with error">*39.484 ns\**</span> | <span title="unvalidated">*54.152 µs\**</span> <span title="validated upfront with error">*77.407 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*4.9525 ns\**</span> | <span title="unvalidated">*48.838 µs\**</span> <span title="validated upfront with error">*38.697 µs\**</span> | <span title="unvalidated">*99.789 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.97%\**</span> <span title="prepend">*2.25%\**</span> | 1.62% | 69.57% | 80.42% | 79.02% | 10.51% |
| [bincode 2.0.0-rc][bincode] | 8.29% | 13.98% | 100.00% | 96.35% | 92.11% | 92.54% |
| [bincode 1.3.3][bincode1] | 3.90% | 3.11% | 100.00% | 96.35% | 92.11% | 99.56% |
| [bitcode 0.6.3][bitcode] | 14.02% | 18.60% | 100.00% | 100.00% | 100.00% | 57.04% |
| [borsh 1.5.1][borsh] | 3.26% | 3.57% | 100.00% | 96.35% | 92.11% | 96.21% |
| [capnp 0.19.7][capnp] | 3.50% | † | 42.86% | 72.68% | 81.37% | 9.37% |
| [cbor4ii 0.3.3][cbor4ii] | 1.94% | 0.29% | 45.71% | 68.88% | 72.86% | 8.04% |
| [ciborium 0.2.2][ciborium] | 0.29% | 0.13% | 45.72% | 68.87% | 72.84% | 8.03% |
| [databuf 0.5.0][databuf] | 8.29% | 2.78% | 100.00% | 96.35% | 92.11% | 98.20% |
| [dlhn 0.1.7][dlhn] | 3.11% | 2.20% | 100.00% | 96.35% | 92.11% | 98.34% |
| [flatbuffers 24.3.25][flatbuffers] | 23.12% | † | 100.00% | 96.35% | 92.11% | 97.00% |
| [msgpacker 0.4.3][msgpacker] | 1.06% | 2.86% | 80.00% | 85.54% | 81.87% | 67.93% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.46% | 73.85% | 79.81% | 77.09% | 10.04% |
| [nanoserde 0.1.37][nanoserde] | 14.90% | 13.38% | 100.00% | 96.35% | 92.11% | 94.94% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.91% | 3.70% | 100.00% | 96.35% | 92.11% | 99.05% |
| [postcard 1.0.10][postcard] | 41.42% | 11.08% | 100.00% | 96.35% | 92.11% | 95.46% |
| [pot 3.0.1][pot] | 0.52% | 0.20% | 59.27% | 76.05% | 71.86% | 9.16% |
| [prost 0.13.2][prost] | <span title="encode">*2.57%\**</span> <span title="populate + encode">*2.38%\**</span> | 1.09% | 68.57% | 77.75% | 76.67% | 10.60% |
| [rkyv 0.8.5][rkyv] | 83.27% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.67%\**</span> | 100.00% | 96.35% | 92.11% | 100.00% |
| [rmp-serde 1.3.0][rmp-serde] | 1.30% | 0.85% | 73.85% | 79.79% | 77.04% | 11.10% |
| [ron 0.8.1][ron] | 0.12% | 0.06% | 27.04% | 57.77% | 60.50% | 4.95% |
| [savefile 0.17.7][savefile] | 83.22% | 62.12% | 100.00% | 96.35% | 92.11% | 96.29% |
| [serde-brief 0.1.0][serde-brief] | 0.88% | 0.40% | 38.10% | 64.58% | 72.23% | 7.97% |
| [serde_bare 0.5.0][serde_bare] | 3.21% | 3.11% | 100.00% | 96.35% | 92.11% | 95.27% |
| [serde_cbor 0.11.2][serde_cbor] | 0.58% | 0.32% | 45.72% | 68.87% | 72.84% | 7.76% |
| [serde_json 1.0.128][serde_json] | 0.23% | 0.17% | 22.91% | 54.17% | 57.34% | 4.79% |
| [serialization 0.2.8][serialization] | 57.88% | 74.79% | 100.00% | 96.35% | 92.11% | 96.27% |
| [speedy 0.8.7][speedy] | 83.29% | 61.97% | 100.00% | 96.35% | 92.11% | 98.76% |
| [wiring 0.2.2][wiring] | 100.00% | 41.88% | 100.00% | 96.34% | 92.11% | 97.53% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.80%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.32%\**</span> <span title="validated upfront with error">*3.13%\**</span> | <span title="unvalidated">*71.46%\**</span> <span title="validated upfront with error">*49.99%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.98%\**</span> | <span title="unvalidated">*79.24%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.8][serialization] | 691.80 µs | † | † | † | † | † |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [serialization 0.2.8][serialization] | 100.00% | † | † | † | † | † |

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
[serialization]: https://crates.io/crates/serialization/0.2.8
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
