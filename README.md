# hyw - 合义维

[![GitHub License](https://img.shields.io/github/license/PRO-2684/hyw?logo=opensourceinitiative)](https://github.com/PRO-2684/hyw/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/hyw/release.yml?logo=githubactions)](https://github.com/PRO-2684/hyw/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/hyw?logo=githubactions)](https://github.com/PRO-2684/hyw/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/hyw/total?logo=github)](https://github.com/PRO-2684/hyw/releases)
[![Crates.io Version](https://img.shields.io/crates/v/hyw?logo=rust)](https://crates.io/crates/hyw)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/hyw?logo=rust)](https://crates.io/crates/hyw)
[![docs.rs](https://img.shields.io/docsrs/hyw?logo=rust)](https://docs.rs/hyw)

> 穷举排列组 **合**，寻找最接近的语 **义** 于上千 **维** 度中。

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall hyw
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/hyw/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install hyw
```

## 📊 Embedding Data

You can download pre-computed data from [this Release](https://github.com/PRO-2684/hyw/releases/tag/_data-251226), or generate one via the [`hyw-embed`](./hyw-embed) crate.

## 📖 Usage

```shell
$ hyw -h
Usage: hyw -k <api-key> [-m <map-path>]

Querying embeddings for hyw.

Options:
  -k, --api-key     key for SiliconFlow API
  -m, --map-path    path to the embedding map file
  -h, --help        display usage information
```

## 💡 Examples

```shell
$ hyw -k sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Loading embedding map from ../data/hyw.postcard
Embedding map has been loaded!

Enter search query (or press Enter to exit): 电流的原理
#1: 荷移为 (Distance: 1.1049)
#2: 荷移位 (Distance: 1.1298)
#3: 荷易位 (Distance: 1.1337)
#4: 詥移帏 (Distance: 1.1353)
#5: 翯移磈 (Distance: 1.1353)

Enter search query (or press Enter to exit): 长江中飘散着臭味
#1: 河异味 (Distance: 0.8056)
#2: 嗬异味 (Distance: 0.9125)
#3: 翯异味 (Distance: 0.9142)
#4: 翮异味 (Distance: 0.9142)
#5: 菏异味 (Distance: 0.9147)

Enter search query (or press Enter to exit): 花凋零了
#1: 荷已萎 (Distance: 0.9007)
#2: 荷矣萎 (Distance: 0.9700)
#3: 呵已萎 (Distance: 0.9754)
#4: 荷已微 (Distance: 0.9954)
#5: 荷亦萎 (Distance: 0.9985)

Enter search query (or press Enter to exit): 下巴脱臼
#1: 颌易位 (Distance: 0.9067)
#2: 颌移位 (Distance: 0.9143)
#3: 颌异位 (Distance: 0.9153)
#4: 颌已萎 (Distance: 0.9544)
#5: 颌逸萎 (Distance: 0.9582)

Enter search query (or press Enter to exit):
Exiting. Goodbye!
```

## 🎉 Credits

TODO
