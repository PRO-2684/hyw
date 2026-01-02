# hyw - 合义维

[![GitHub License](https://img.shields.io/github/license/PRO-2684/hyw?logo=opensourceinitiative)](https://github.com/PRO-2684/hyw/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/hyw/release.yml?logo=githubactions)](https://github.com/PRO-2684/hyw/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/hyw?logo=githubactions)](https://github.com/PRO-2684/hyw/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/hyw/total?logo=github)](https://github.com/PRO-2684/hyw/releases)
[![Crates.io Version](https://img.shields.io/crates/v/hyw?logo=rust)](https://crates.io/crates/hyw)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/hyw?logo=rust)](https://crates.io/crates/hyw)
[![docs.rs](https://img.shields.io/docsrs/hyw?logo=rust)](https://docs.rs/hyw)

> 穷举排列组 **合**，寻找最接近的语 **义** 于上千 **维** 度中。

## ℹ️ Introduction

### `hyw` Combination & `hyw` Sequence

A valid `hyw` combination consists of three characters, with the pinyin for each character being `he`, `yi` and `wei`. To systematically cover all possibilities, we can generate all possible `hyw` combinations through permutations. The `hyw` sequence is defined as the ordered list of all `hyw` combinations. For detailed implementation and character set, you can refer to [`hyw-base` crate](./hyw-base/src/lib.rs).

### `hyw` (合义维)

`hyw` (合义维) is a CLI tool designed to explore the `hyw` sequence. It can:

- List the entire `hyw` sequence (`list`)
- Query given `hyw` combinations' indices in the `hyw` sequence and vice versa (`query`)
- Search the closest `hyw` sequence to input in terms of semantics (`search`)

It was heavily inspired by Liu Cixin's science fiction novel "Poetry Cloud".[^1]

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
Usage: hyw [<action>] [-k <api-key>] [-m <map-path>]

合义维

Positional Arguments:
  action            the action to take, can be 'list'/'l', 'query'/'q' or
                    'search'/'s' (default)

Options:
  -k, --api-key     key for SiliconFlow API, required for search, using
                    environment variable SILICONFLOW_API_KEY if not provided
  -m, --map-path    path to the embedding map file, default is "./hyw.postcard"
  -h, --help        display usage information
```

Note that the api key is only required for "search" action. If you don't have one, simply create an account and get one from their [API keys page](https://cloud.siliconflow.cn/me/account/ak). Their [embedding API](https://docs.siliconflow.cn/en/api-reference/embeddings/create-embeddings) is free to use.

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

## 🔗 References

[^1]: 刘慈欣. 诗云[J]. 科幻世界, 2003(3).
[^2]: 害怕的狗XGGGGGGGG258. 这是我个人的一小步，却是CCB的一大步[EB/OL]. Bilibili, (2025-02-22)[2026-01-01]. https://www.bilibili.com/video/BV1c3PuewEmu
