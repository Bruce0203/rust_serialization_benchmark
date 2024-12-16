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

## Last updated: 2024-12-16 3:31:41

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
| [bilrost 0.1010.0][bilrost] | <span title="encode">*703.72 µs\**</span> <span title="prepend">*620.18 µs\**</span> | 3.2489 ms | 874632 | 355446 | 311723 | 5.1742 ms |
| [bincode 2.0.0-rc][bincode] | 299.85 µs | 2.5181 ms | 741295 | 303944 | 257153 | 3.9682 ms |
| [bincode 1.3.3][bincode1] | 525.80 µs | 1.9841 ms | 1045784 | 373127 | 311761 | 4.8766 ms |
| [bitcode 0.6.3][bitcode] | 139.07 µs | 1.4641 ms | 703710 | 288826 | 229755 | 2.4400 ms |
| [borsh 1.5.1][borsh] | 548.76 µs | 2.2287 ms | 885780 | 362204 | 286514 | 4.4488 ms |
| [capnp 0.19.7][capnp] | 774.42 µs | † | 1443216 | 513986 | 428649 | 6.8095 ms |
| [cbor4ii 0.3.3][cbor4ii] | 589.37 µs | 5.0166 ms | 1407835 | 403440 | 324081 | 4.8246 ms |
| [ciborium 0.2.2][ciborium] | 3.2901 ms | 12.021 ms | 1407835 | 403440 | 324081 | 4.8377 ms |
| [databuf 0.5.0][databuf] | 263.70 µs | 2.0605 ms | 765778 | 311715 | 264630 | 4.0596 ms |
| [dlhn 0.1.7][dlhn] | 738.55 µs | 2.5313 ms | 724953 | 301446 | 253629 | 3.7479 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0101 ms | † | 1276368 | 468539 | 388832 | 5.1536 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2412 ms | 2.6113 ms | 764996 | 315291 | 264898 | 3.9031 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3784 ms | 4.1151 ms | 818669 | 332556 | 285514 | 4.3607 ms |
| [nanoserde 0.1.37][nanoserde] | 263.42 µs | 2.1332 ms | 1045784 | 373127 | 311761 | 4.6321 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 648.08 µs | 2.2232 ms | 765778 | 311743 | 264518 | 3.8373 ms |
| [postcard 1.0.10][postcard] | 421.24 µs | 2.2298 ms | 724953 | 302399 | 253747 | 3.7902 ms |
| [pot 3.0.1][pot] | 2.2603 ms | 6.3385 ms | 971922 | 372513 | 304122 | 4.7731 ms |
| [prost 0.13.2][prost] | <span title="encode">*923.86 µs\**</span> <span title="populate + encode">*2.4833 ms\**</span> | 3.3004 ms | 884628 | 363130 | 315494 | 5.0849 ms |
| [rkyv 0.8.5][rkyv] | 244.79 µs | <span title="unvalidated">*1.6010 ms\**</span> <span title="validated upfront with error">*2.1947 ms\**</span> | 1011488 | 393526 | 326517 | 4.9877 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3036 ms | 3.2403 ms | 784997 | 325384 | 278219 | 4.1366 ms |
| [ron 0.8.1][ron] | 11.306 ms | 15.684 ms | 1607459 | 449158 | 349713 | 5.9751 ms |
| [savefile 0.17.7][savefile] | 192.42 µs | 2.2049 ms | 1045800 | 373140 | 311777 | 4.6809 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5235 ms | 4.9586 ms | 1584946 | 413733 | 341439 | 4.9735 ms |
| [serde_bare 0.5.0][serde_bare] | 668.55 µs | 2.1352 ms | 765778 | 311715 | 264630 | 3.9638 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0336 ms | 5.2646 ms | 1407835 | 403440 | 324081 | 4.9248 ms |
| [serde_json 1.0.128][serde_json] | 3.8523 ms | 5.3944 ms | 1827461 | 470560 | 361090 | 5.7953 ms |
| [serialization 0.2.7][serialization] | 1.1443 ms | 7.3136 ms | 765778 | 305641 | 259060 | 4.0753 ms |
| [simd-json 0.13.10][simd-json] | 2.0763 ms | 4.7136 ms | 1827461 | 470560 | 361090 | 5.8197 ms |
| [speedy 0.8.7][speedy] | 195.45 µs | 1.8021 ms | 885780 | 362204 | 286514 | 4.2508 ms |
| [wiring 0.2.2][wiring] | 191.21 µs | 1.9898 ms | 1045784 | 337930 | 276188 | 4.1871 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.053 ns\**</span> | <span title="validated on-demand with error">*168.44 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4739 ns\**</span> <span title="validated upfront with error">*1.9893 ms\**</span> | <span title="unvalidated">*49.043 µs\**</span> <span title="validated upfront with error">*2.1449 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*617.13 µs\**</span> | <span title="unvalidated">*10.512 µs\**</span> <span title="validated upfront with error">*621.36 µs\**</span> | <span title="unvalidated">*7.5288 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*19.76%\**</span> <span title="prepend">*22.42%\**</span> | 45.06% | 80.46% | 81.26% | 73.70% | 47.16% |
| [bincode 2.0.0-rc][bincode] | 46.38% | 58.14% | 94.93% | 95.03% | 89.35% | 61.49% |
| [bincode 1.3.3][bincode1] | 26.45% | 73.79% | 67.29% | 77.41% | 73.70% | 50.03% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.34% | 65.69% | 79.45% | 79.74% | 80.19% | 54.85% |
| [capnp 0.19.7][capnp] | 17.96% | † | 48.76% | 56.19% | 53.60% | 35.83% |
| [cbor4ii 0.3.3][cbor4ii] | 23.60% | 29.19% | 49.99% | 71.59% | 70.89% | 50.57% |
| [ciborium 0.2.2][ciborium] | 4.23% | 12.18% | 49.99% | 71.59% | 70.89% | 50.44% |
| [databuf 0.5.0][databuf] | 52.74% | 71.06% | 91.89% | 92.66% | 86.82% | 60.10% |
| [dlhn 0.1.7][dlhn] | 18.83% | 57.84% | 97.07% | 95.81% | 90.59% | 65.10% |
| [flatbuffers 24.3.25][flatbuffers] | 13.77% | † | 55.13% | 61.64% | 59.09% | 47.35% |
| [msgpacker 0.4.3][msgpacker] | 11.20% | 56.07% | 91.99% | 91.61% | 86.73% | 62.51% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.59% | 35.58% | 85.96% | 86.85% | 80.47% | 55.95% |
| [nanoserde 0.1.37][nanoserde] | 52.79% | 68.63% | 67.29% | 77.41% | 73.70% | 52.68% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.46% | 65.86% | 91.89% | 92.65% | 86.86% | 63.59% |
| [postcard 1.0.10][postcard] | 33.01% | 65.66% | 97.07% | 95.51% | 90.54% | 64.38% |
| [pot 3.0.1][pot] | 6.15% | 23.10% | 72.40% | 77.53% | 75.55% | 51.12% |
| [prost 0.13.2][prost] | <span title="encode">*15.05%\**</span> <span title="populate + encode">*5.60%\**</span> | 44.36% | 79.55% | 79.54% | 72.82% | 47.99% |
| [rkyv 0.8.5][rkyv] | 56.81% | <span title="unvalidated">*91.45%\**</span> <span title="validated upfront with error">*66.71%\**</span> | 69.57% | 73.39% | 70.37% | 48.92% |
| [rmp-serde 1.3.0][rmp-serde] | 10.67% | 45.18% | 89.64% | 88.76% | 82.58% | 58.99% |
| [ron 0.8.1][ron] | 1.23% | 9.33% | 43.78% | 64.30% | 65.70% | 40.84% |
| [savefile 0.17.7][savefile] | 72.27% | 66.40% | 67.29% | 77.40% | 73.69% | 52.13% |
| [serde-brief 0.1.0][serde-brief] | 9.13% | 29.53% | 44.40% | 69.81% | 67.29% | 49.06% |
| [serde_bare 0.5.0][serde_bare] | 20.80% | 68.57% | 91.89% | 92.66% | 86.82% | 61.56% |
| [serde_cbor 0.11.2][serde_cbor] | 6.84% | 27.81% | 49.99% | 71.59% | 70.89% | 49.55% |
| [serde_json 1.0.128][serde_json] | 3.61% | 27.14% | 38.51% | 61.38% | 63.63% | 42.10% |
| [serialization 0.2.7][serialization] | 12.15% | 20.02% | 91.89% | 94.50% | 88.69% | 59.87% |
| [simd-json 0.13.10][simd-json] | 6.70% | 31.06% | 38.51% | 61.38% | 63.63% | 41.93% |
| [speedy 0.8.7][speedy] | 71.15% | 81.24% | 79.45% | 79.74% | 80.19% | 57.40% |
| [wiring 0.2.2][wiring] | 72.73% | 73.58% | 67.29% | 85.47% | 83.19% | 58.27% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*6.24%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.43%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.69%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6475 ms\**</span> <span title="prepend">*8.7568 ms\**</span> | 9.4141 ms | 8625005 | 6443961 | 6231572 | 71.032 ms |
| [bincode 2.0.0-rc][bincode] | 2.8623 ms | 1.3912 ms | 6000005 | 5378497 | 5345897 | 7.4690 ms |
| [bincode 1.3.3][bincode1] | 5.1194 ms | 5.7905 ms | 6000008 | 5378500 | 5345890 | 7.5147 ms |
| [bitcode 0.6.3][bitcode] | 1.4000 ms | 796.44 µs | 6000006 | 5182295 | 4923880 | 12.794 ms |
| [borsh 1.5.1][borsh] | 6.0388 ms | 4.3193 ms | 6000004 | 5378496 | 5345889 | 7.5641 ms |
| [capnp 0.19.7][capnp] | 5.3838 ms | † | 14000088 | 7130367 | 6051062 | 82.495 ms |
| [cbor4ii 0.3.3][cbor4ii] | 10.026 ms | 51.601 ms | 13125016 | 7524114 | 6757967 | 91.210 ms |
| [ciborium 0.2.2][ciborium] | 68.019 ms | 121.36 ms | 13122324 | 7524660 | 6759658 | 92.392 ms |
| [databuf 0.5.0][databuf] | 2.3975 ms | 5.3187 ms | 6000003 | 5378495 | 5345900 | 7.6270 ms |
| [dlhn 0.1.7][dlhn] | 6.4059 ms | 7.3654 ms | 6000003 | 5378495 | 5345900 | 7.5976 ms |
| [flatbuffers 24.3.25][flatbuffers] | 864.57 µs | † | 6000024 | 5378434 | 5345910 | 7.4589 ms |
| [msgpacker 0.4.3][msgpacker] | 18.385 ms | 5.9101 ms | 7500005 | 6058442 | 6014337 | 9.6397 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 119.08 ms | 32.857 ms | 8125037 | 6493484 | 6386940 | 69.296 ms |
| [nanoserde 0.1.37][nanoserde] | 1.8882 ms | 1.1035 ms | 6000008 | 5378500 | 5345890 | 7.3818 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.7845 ms | 4.5381 ms | 6000004 | 5378496 | 5345889 | 7.4989 ms |
| [postcard 1.0.10][postcard] | 489.82 µs | 1.3223 ms | 6000003 | 5378495 | 5345900 | 7.8262 ms |
| [pot 3.0.1][pot] | 38.381 ms | 69.908 ms | 10122342 | 6814618 | 6852251 | 81.780 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7488 ms\**</span> <span title="populate + encode">*8.4883 ms\**</span> | 10.839 ms | 8750000 | 6665735 | 6421871 | 71.472 ms |
| [rkyv 0.8.5][rkyv] | 237.43 µs | <span title="unvalidated">*197.91 µs\**</span> <span title="validated upfront with error">*197.79 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.3771 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.369 ms | 18.020 ms | 8125006 | 6494876 | 6391037 | 67.780 ms |
| [ron 0.8.1][ron] | 176.71 ms | 236.55 ms | 22192885 | 8970395 | 8138755 | 150.50 ms |
| [savefile 0.17.7][savefile] | 237.34 µs | 238.02 µs | 6000024 | 5378513 | 5345893 | 7.4225 ms |
| [serde-brief 0.1.0][serde-brief] | 22.816 ms | 39.725 ms | 15750015 | 8024540 | 6816643 | 93.396 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2693 ms | 5.4028 ms | 6000003 | 5378495 | 5345900 | 7.5075 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.252 ms | 47.579 ms | 13122324 | 7524660 | 6759658 | 90.382 ms |
| [serde_json 1.0.128][serde_json] | 87.224 ms | 85.474 ms | 26192883 | 9566084 | 8586741 | 155.02 ms |
| [serialization 0.2.7][serialization] | 1.3672 ms | 2.8680 ms | 6000003 | 5378495 | 5345900 | 7.4327 ms |
| [simd-json 0.13.10][simd-json] | 53.175 ms | 72.212 ms | 26192883 | 9566084 | 8586741 | 154.76 ms |
| [speedy 0.8.7][speedy] | 238.12 µs | 237.52 µs | 6000004 | 5378496 | 5345889 | 7.4477 ms |
| [wiring 0.2.2][wiring] | 197.54 µs | 336.89 µs | 6000008 | 5378952 | 5345894 | 7.4653 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*105.98 ns\**</span> | <span title="validated on-demand with error">*2.1927 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*39.968 ns\**</span> | <span title="unvalidated">*77.349 µs\**</span> <span title="validated upfront with error">*77.379 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*5.2888 ns\**</span> | <span title="unvalidated">*48.400 µs\**</span> <span title="validated upfront with error">*38.685 µs\**</span> | <span title="unvalidated">*99.145 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.97%\**</span> <span title="prepend">*2.26%\**</span> | 2.10% | 69.57% | 80.42% | 79.02% | 10.39% |
| [bincode 2.0.0-rc][bincode] | 6.90% | 14.22% | 100.00% | 96.35% | 92.11% | 98.77% |
| [bincode 1.3.3][bincode1] | 3.86% | 3.42% | 100.00% | 96.35% | 92.11% | 98.17% |
| [bitcode 0.6.3][bitcode] | 14.11% | 24.83% | 100.00% | 100.00% | 100.00% | 57.66% |
| [borsh 1.5.1][borsh] | 3.27% | 4.58% | 100.00% | 96.35% | 92.11% | 97.53% |
| [capnp 0.19.7][capnp] | 3.67% | † | 42.86% | 72.68% | 81.37% | 8.94% |
| [cbor4ii 0.3.3][cbor4ii] | 1.97% | 0.38% | 45.71% | 68.88% | 72.86% | 8.09% |
| [ciborium 0.2.2][ciborium] | 0.29% | 0.16% | 45.72% | 68.87% | 72.84% | 7.98% |
| [databuf 0.5.0][databuf] | 8.24% | 3.72% | 100.00% | 96.35% | 92.11% | 96.72% |
| [dlhn 0.1.7][dlhn] | 3.08% | 2.69% | 100.00% | 96.35% | 92.11% | 97.10% |
| [flatbuffers 24.3.25][flatbuffers] | 22.85% | † | 100.00% | 96.35% | 92.11% | 98.90% |
| [msgpacker 0.4.3][msgpacker] | 1.07% | 3.35% | 80.00% | 85.54% | 81.87% | 76.53% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.17% | 0.60% | 73.85% | 79.81% | 77.09% | 10.65% |
| [nanoserde 0.1.37][nanoserde] | 10.46% | 17.92% | 100.00% | 96.35% | 92.11% | 99.94% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.13% | 4.36% | 100.00% | 96.35% | 92.11% | 98.38% |
| [postcard 1.0.10][postcard] | 40.33% | 14.96% | 100.00% | 96.35% | 92.11% | 94.26% |
| [pot 3.0.1][pot] | 0.51% | 0.28% | 59.27% | 76.05% | 71.86% | 9.02% |
| [prost 0.13.2][prost] | <span title="encode">*2.55%\**</span> <span title="populate + encode">*2.33%\**</span> | 1.82% | 68.57% | 77.75% | 76.67% | 10.32% |
| [rkyv 0.8.5][rkyv] | 83.20% | <span title="unvalidated">*99.94%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 100.00% |
| [rmp-serde 1.3.0][rmp-serde] | 1.29% | 1.10% | 73.85% | 79.79% | 77.04% | 10.88% |
| [ron 0.8.1][ron] | 0.11% | 0.08% | 27.04% | 57.77% | 60.50% | 4.90% |
| [savefile 0.17.7][savefile] | 83.23% | 83.10% | 100.00% | 96.35% | 92.11% | 99.39% |
| [serde-brief 0.1.0][serde-brief] | 0.87% | 0.50% | 38.10% | 64.58% | 72.23% | 7.90% |
| [serde_bare 0.5.0][serde_bare] | 3.15% | 3.66% | 100.00% | 96.35% | 92.11% | 98.26% |
| [serde_cbor 0.11.2][serde_cbor] | 0.56% | 0.42% | 45.72% | 68.87% | 72.84% | 8.16% |
| [serde_json 1.0.128][serde_json] | 0.23% | 0.23% | 22.91% | 54.17% | 57.34% | 4.76% |
| [serialization 0.2.7][serialization] | 14.45% | 6.90% | 100.00% | 96.35% | 92.11% | 99.25% |
| [simd-json 0.13.10][simd-json] | 0.37% | 0.27% | 22.91% | 54.17% | 57.34% | 4.77% |
| [speedy 0.8.7][speedy] | 82.96% | 83.27% | 100.00% | 96.35% | 92.11% | 99.05% |
| [wiring 0.2.2][wiring] | 100.00% | 58.71% | 100.00% | 96.34% | 92.11% | 98.82% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.17%\**</span> | <span title="validated on-demand with error">*1.76%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*3.09%\**</span> | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*49.99%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.38%\**</span> | <span title="unvalidated">*79.93%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*939.15 µs\**</span> <span title="prepend">*837.44 µs\**</span> | 3.2302 ms | 489348 | 281173 | 249546 | 3.0820 ms |
| [bincode 2.0.0-rc][bincode] | 274.73 µs | 2.0851 ms | 367413 | 221291 | 206273 | 2.4725 ms |
| [bincode 1.3.3][bincode1] | 599.53 µs | 1.8819 ms | 569975 | 240525 | 232423 | 3.7450 ms |
| [bitcode 0.6.3][bitcode] | 132.97 µs | 1.2819 ms | 327688 | 200947 | 182736 | 752.10 µs |
| [borsh 1.5.1][borsh] | 554.07 µs | 1.8195 ms | 446595 | 234236 | 210008 | 2.4760 ms |
| [capnp 0.19.7][capnp] | 467.10 µs | † | 803896 | 335606 | 280851 | 3.9335 ms |
| [cbor4ii 0.3.3][cbor4ii] | 779.25 µs | 4.8267 ms | 1109831 | 344745 | 274514 | 3.8249 ms |
| [ciborium 0.2.2][ciborium] | 3.8653 ms | 10.301 ms | 1109821 | 344751 | 274526 | 3.8049 ms |
| [databuf 0.5.0][databuf] | 325.62 µs | 1.7430 ms | 356311 | 213062 | 198488 | 2.4236 ms |
| [dlhn 0.1.7][dlhn] | 797.03 µs | 2.6207 ms | 366496 | 220600 | 205683 | 2.4701 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.2721 ms | † | 844168 | 345696 | 294015 | 3.8267 ms |
| [msgpacker 0.4.3][msgpacker] | 962.05 µs | 2.8855 ms | 391251 | 236877 | 220476 | 2.6398 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1269 ms | 3.9940 ms | 449745 | 252432 | 231110 | 2.7423 ms |
| [nanoserde 0.1.37][nanoserde] | 272.52 µs | 1.9540 ms | 567975 | 239930 | 232419 | 2.9101 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 598.40 µs | 2.0132 ms | 356311 | 212976 | 198524 | 2.3715 ms |
| [postcard 1.0.10][postcard] | 449.50 µs | 2.0648 ms | 367489 | 221913 | 207344 | 2.5251 ms |
| [pot 3.0.1][pot] | 2.3765 ms | 6.0222 ms | 599125 | 299158 | 247693 | 3.1479 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.0763 ms\**</span> <span title="populate + encode">*2.7746 ms\**</span> | 3.4633 ms | 596811 | 305319 | 269310 | 3.4531 ms |
| [rkyv 0.8.5][rkyv] | 346.42 µs | <span title="unvalidated">*1.5243 ms\**</span> <span title="validated upfront with error">*2.0549 ms\**</span> | 603776 | 254776 | 220087 | 2.7348 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3582 ms | 3.0443 ms | 424533 | 245214 | 226188 | 2.6764 ms |
| [ron 0.8.1][ron] | 7.1939 ms | 16.990 ms | 1465223 | 434935 | 343338 | 5.8532 ms |
| [savefile 0.17.7][savefile] | 213.27 µs | 1.8600 ms | 566991 | 239361 | 232013 | 2.8630 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3642 ms | 5.4288 ms | 1276014 | 373898 | 293679 | 4.0528 ms |
| [serde_bare 0.5.0][serde_bare] | 737.51 µs | 2.3413 ms | 356311 | 213062 | 198488 | 2.3749 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8574 ms | 4.7716 ms | 1109821 | 344751 | 274526 | 3.8248 ms |
| [serde_json 1.0.128][serde_json] | 3.6305 ms | 6.3128 ms | 1623191 | 466527 | 359623 | 6.0346 ms |
| [serialization 0.2.7][serialization] | 1.2039 ms | 5.7481 ms | 356311 | 211955 | 195923 | 2.3470 ms |
| [simd-json 0.13.10][simd-json] | 2.2203 ms | 4.5767 ms | 1623191 | 466527 | 359623 | 6.0299 ms |
| [speedy 0.8.7][speedy] | 268.88 µs | 1.6016 ms | 449595 | 234970 | 210361 | 2.4847 ms |
| [wiring 0.2.2][wiring] | 209.67 µs | 1.8337 ms | 566975 | 247810 | 225259 | 2.9143 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.121 ns\**</span> | <span title="validated on-demand with error">*409.18 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4735 ns\**</span> <span title="validated upfront with error">*2.1850 ms\**</span> | <span title="unvalidated">*1.3523 µs\**</span> <span title="validated upfront with error">*2.2058 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*536.70 µs\**</span> | <span title="unvalidated">*163.02 ns\**</span> <span title="validated upfront with error">*538.79 µs\**</span> | <span title="unvalidated">*724.87 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*14.16%\**</span> <span title="prepend">*15.88%\**</span> | 39.68% | 66.96% | 71.47% | 73.23% | 24.40% |
| [bincode 2.0.0-rc][bincode] | 48.40% | 61.48% | 89.19% | 90.81% | 88.59% | 30.42% |
| [bincode 1.3.3][bincode1] | 22.18% | 68.12% | 57.49% | 83.55% | 78.62% | 20.08% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 24.00% | 70.45% | 73.37% | 85.79% | 87.01% | 30.38% |
| [capnp 0.19.7][capnp] | 28.47% | † | 40.76% | 59.88% | 65.07% | 19.12% |
| [cbor4ii 0.3.3][cbor4ii] | 17.06% | 26.56% | 29.53% | 58.29% | 66.57% | 19.66% |
| [ciborium 0.2.2][ciborium] | 3.44% | 12.44% | 29.53% | 58.29% | 66.56% | 19.77% |
| [databuf 0.5.0][databuf] | 40.84% | 73.55% | 91.97% | 94.31% | 92.06% | 31.03% |
| [dlhn 0.1.7][dlhn] | 16.68% | 48.91% | 89.41% | 91.09% | 88.84% | 30.45% |
| [flatbuffers 24.3.25][flatbuffers] | 4.06% | † | 38.82% | 58.13% | 62.15% | 19.65% |
| [msgpacker 0.4.3][msgpacker] | 13.82% | 44.43% | 83.75% | 84.83% | 82.88% | 28.49% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.59% | 32.10% | 72.86% | 79.60% | 79.07% | 27.43% |
| [nanoserde 0.1.37][nanoserde] | 48.79% | 65.60% | 57.69% | 83.75% | 78.62% | 25.84% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.22% | 63.67% | 91.97% | 94.35% | 92.05% | 31.71% |
| [postcard 1.0.10][postcard] | 29.58% | 62.08% | 89.17% | 90.55% | 88.13% | 29.78% |
| [pot 3.0.1][pot] | 5.60% | 21.29% | 54.69% | 67.17% | 73.78% | 23.89% |
| [prost 0.13.2][prost] | <span title="encode">*12.35%\**</span> <span title="populate + encode">*4.79%\**</span> | 37.01% | 54.91% | 65.82% | 67.85% | 21.78% |
| [rkyv 0.8.5][rkyv] | 38.38% | <span title="unvalidated">*84.10%\**</span> <span title="validated upfront with error">*62.38%\**</span> | 54.27% | 78.87% | 83.03% | 27.50% |
| [rmp-serde 1.3.0][rmp-serde] | 9.79% | 42.11% | 77.19% | 81.95% | 80.79% | 28.10% |
| [ron 0.8.1][ron] | 1.85% | 7.55% | 22.36% | 46.20% | 53.22% | 12.85% |
| [savefile 0.17.7][savefile] | 62.35% | 68.92% | 57.79% | 83.95% | 78.76% | 26.27% |
| [serde-brief 0.1.0][serde-brief] | 9.75% | 23.61% | 25.68% | 53.74% | 62.22% | 18.56% |
| [serde_bare 0.5.0][serde_bare] | 18.03% | 54.75% | 91.97% | 94.31% | 92.06% | 31.67% |
| [serde_cbor 0.11.2][serde_cbor] | 7.16% | 26.87% | 29.53% | 58.29% | 66.56% | 19.66% |
| [serde_json 1.0.128][serde_json] | 3.66% | 20.31% | 20.19% | 43.07% | 50.81% | 12.46% |
| [serialization 0.2.7][serialization] | 11.04% | 22.30% | 91.97% | 94.81% | 93.27% | 32.05% |
| [simd-json 0.13.10][simd-json] | 5.99% | 28.01% | 20.19% | 43.07% | 50.81% | 12.47% |
| [speedy 0.8.7][speedy] | 49.45% | 80.04% | 72.89% | 85.52% | 86.87% | 30.27% |
| [wiring 0.2.2][wiring] | 63.42% | 69.91% | 57.80% | 81.09% | 81.12% | 25.81% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*39.84%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.06%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.4591 ms\**</span> <span title="prepend">*2.5001 ms\**</span> | 8.1315 ms | 1664428 | 1264167 | 1216472 | 11.050 ms |
| [bincode 2.0.0-rc][bincode] | 1.3732 ms | 3.9853 ms | 1372381 | 1091486 | 1037296 | 8.9552 ms |
| [bincode 1.3.3][bincode1] | 4.0353 ms | 4.2349 ms | 1811011 | 1115281 | 1025627 | 9.7037 ms |
| [bitcode 0.6.3][bitcode] | 702.92 µs | 2.3110 ms | 948499 | 857321 | 837658 | 3.0524 ms |
| [borsh 1.5.1][borsh] | 2.8275 ms | 2.8156 ms | 1486162 | 1082357 | 1013550 | 9.5563 ms |
| [capnp 0.19.7][capnp] | 2.1445 ms | † | 2664040 | 1511895 | 1212087 | 13.844 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2461 ms | 18.358 ms | 5878791 | 1655835 | 1431390 | 20.884 ms |
| [ciborium 0.2.2][ciborium] | 22.690 ms | 54.990 ms | 5878653 | 1655791 | 1431560 | 20.705 ms |
| [databuf 0.5.0][databuf] | 1.2833 ms | 3.7001 ms | 1288257 | 1037579 | 984337 | 8.4602 ms |
| [dlhn 0.1.7][dlhn] | 4.9590 ms | 6.5658 ms | 1279599 | 1052061 | 1021161 | 8.2619 ms |
| [flatbuffers 24.3.25][flatbuffers] | 4.7659 ms | † | 2273740 | 1408408 | 1235566 | 12.764 ms |
| [msgpacker 0.4.3][msgpacker] | 2.2604 ms | 6.7802 ms | 1424043 | 1128758 | 1110156 | 9.3678 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 28.955 ms | 17.371 ms | 1728519 | 1247642 | 1233323 | 12.278 ms |
| [nanoserde 0.1.37][nanoserde] | 1.4711 ms | 2.8975 ms | 1770477 | 1108304 | 1029947 | 9.8290 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.0443 ms | 3.2435 ms | 1288257 | 1039269 | 986510 | 8.7761 ms |
| [postcard 1.0.10][postcard] | 1.9325 ms | 4.1967 ms | 1279599 | 1058243 | 1016738 | 8.3558 ms |
| [pot 3.0.1][pot] | 13.764 ms | 29.484 ms | 2544810 | 1447453 | 1268390 | 15.290 ms |
| [prost 0.13.2][prost] | <span title="encode">*4.7882 ms\**</span> <span title="populate + encode">*8.6741 ms\**</span> | 8.5871 ms | 1818378 | 1307777 | 1266311 | 11.393 ms |
| [rkyv 0.8.5][rkyv] | 908.76 µs | <span title="unvalidated">*2.1413 ms\**</span> <span title="validated upfront with error">*2.5768 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.529 ms |
| [rmp-serde 1.3.0][rmp-serde] | 9.8505 ms | 10.732 ms | 1703813 | 1231892 | 1200208 | 10.838 ms |
| [ron 0.8.1][ron] | 36.835 ms | 86.148 ms | 8476284 | 2181196 | 1783971 | 34.307 ms |
| [savefile 0.17.7][savefile] | 819.88 µs | 2.7327 ms | 1750226 | 1101682 | 1027828 | 9.7883 ms |
| [serde-brief 0.1.0][serde-brief] | 6.4037 ms | 22.145 ms | 6796949 | 1754624 | 1533223 | 22.905 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8523 ms | 4.6962 ms | 1288257 | 1037597 | 984356 | 8.4495 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.6225 ms | 21.295 ms | 5878653 | 1655791 | 1431560 | 20.955 ms |
| [serde_json 1.0.128][serde_json] | 19.969 ms | 29.069 ms | 9175594 | 2334253 | 1800713 | 33.873 ms |
| [serialization 0.2.7][serialization] | 4.3230 ms | 9.5214 ms | 1288257 | 1037644 | 980863 | 8.3790 ms |
| [simd-json 0.13.10][simd-json] | 11.421 ms | 26.335 ms | 9175594 | 2334253 | 1800713 | 33.828 ms |
| [speedy 0.8.7][speedy] | 748.79 µs | 2.4294 ms | 1546963 | 1093532 | 1013443 | 9.6704 ms |
| [wiring 0.2.2][wiring] | 635.41 µs | 2.7206 ms | 1750210 | 1129857 | 1058906 | 10.165 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.100 ns\**</span> | <span title="validated on-demand with error">*709.44 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4754 ns\**</span> <span title="validated upfront with error">*5.1005 ms\**</span> | <span title="unvalidated">*2.6291 µs\**</span> <span title="validated upfront with error">*5.0894 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*424.49 µs\**</span> | <span title="unvalidated">*436.08 ns\**</span> <span title="validated upfront with error">*425.08 µs\**</span> | <span title="unvalidated">*235.57 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*14.25%\**</span> <span title="prepend">*25.42%\**</span> | 26.33% | 56.99% | 67.82% | 68.86% | 27.62% |
| [bincode 2.0.0-rc][bincode] | 46.27% | 53.73% | 69.11% | 78.55% | 80.75% | 34.09% |
| [bincode 1.3.3][bincode1] | 15.75% | 50.56% | 52.37% | 76.87% | 81.67% | 31.46% |
| [bitcode 0.6.3][bitcode] | 90.40% | 92.66% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 22.47% | 76.05% | 63.82% | 79.21% | 82.65% | 31.94% |
| [capnp 0.19.7][capnp] | 29.63% | † | 35.60% | 56.71% | 69.11% | 22.05% |
| [cbor4ii 0.3.3][cbor4ii] | 19.57% | 11.66% | 16.13% | 51.78% | 58.52% | 14.62% |
| [ciborium 0.2.2][ciborium] | 2.80% | 3.89% | 16.13% | 51.78% | 58.51% | 14.74% |
| [databuf 0.5.0][databuf] | 49.51% | 57.87% | 73.63% | 82.63% | 85.10% | 36.08% |
| [dlhn 0.1.7][dlhn] | 12.81% | 32.61% | 74.12% | 81.49% | 82.03% | 36.95% |
| [flatbuffers 24.3.25][flatbuffers] | 13.33% | † | 41.72% | 60.87% | 67.80% | 23.91% |
| [msgpacker 0.4.3][msgpacker] | 28.11% | 31.58% | 66.61% | 75.95% | 75.45% | 32.58% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.19% | 12.33% | 54.87% | 68.72% | 67.92% | 24.86% |
| [nanoserde 0.1.37][nanoserde] | 43.19% | 73.90% | 53.57% | 77.35% | 81.33% | 31.06% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.87% | 66.02% | 73.63% | 82.49% | 84.91% | 34.78% |
| [postcard 1.0.10][postcard] | 32.88% | 51.02% | 74.12% | 81.01% | 82.39% | 36.53% |
| [pot 3.0.1][pot] | 4.62% | 7.26% | 37.27% | 59.23% | 66.04% | 19.96% |
| [prost 0.13.2][prost] | <span title="encode">*13.27%\**</span> <span title="populate + encode">*7.33%\**</span> | 24.94% | 52.16% | 65.56% | 66.15% | 26.79% |
| [rkyv 0.8.5][rkyv] | 69.92% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.10%\**</span> | 46.75% | 63.41% | 70.75% | 24.36% |
| [rmp-serde 1.3.0][rmp-serde] | 6.45% | 19.95% | 55.67% | 69.59% | 69.79% | 28.17% |
| [ron 0.8.1][ron] | 1.73% | 2.49% | 11.19% | 39.31% | 46.95% | 8.90% |
| [savefile 0.17.7][savefile] | 77.50% | 78.36% | 54.19% | 77.82% | 81.50% | 31.18% |
| [serde-brief 0.1.0][serde-brief] | 9.92% | 9.67% | 13.95% | 48.86% | 54.63% | 13.33% |
| [serde_bare 0.5.0][serde_bare] | 13.10% | 45.60% | 73.63% | 82.63% | 85.10% | 36.13% |
| [serde_cbor 0.11.2][serde_cbor] | 6.60% | 10.06% | 16.13% | 51.78% | 58.51% | 14.57% |
| [serde_json 1.0.128][serde_json] | 3.18% | 7.37% | 10.34% | 36.73% | 46.52% | 9.01% |
| [serialization 0.2.7][serialization] | 14.70% | 22.49% | 73.63% | 82.62% | 85.40% | 36.43% |
| [simd-json 0.13.10][simd-json] | 5.56% | 8.13% | 10.34% | 36.73% | 46.52% | 9.02% |
| [speedy 0.8.7][speedy] | 84.86% | 88.14% | 61.31% | 78.40% | 82.65% | 31.56% |
| [wiring 0.2.2][wiring] | 100.00% | 78.71% | 54.19% | 75.88% | 79.11% | 30.03% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*61.47%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.59%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
