<div align="center">

# 🇸🇦 pizza-analysis-arabic

**Arabic text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--arabic-blue)](https://github.com/pizza-rs/analysis-arabic)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Arabic language analysis with orthographic normalization, light stemming, and stop words.
Handles Arabic script variations including Alef forms, Taa Marbuta, and Hamza normalization.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `arabic_normalization` | Normalize Alef/Ya/Taa Marbuta/Tatweel/Hamza |
| TokenFilter | `arabic_stem` | Light Arabic stemmer (prefix/suffix removal) |
| TokenFilter | `arabic_stop` | Arabic stop words (119 entries) |
| Analyzer | `arabic` | Full pipeline: lowercase → normalization → stem → stop |

### Normalization Rules

| Input | Output | Rule |
|:------|:-------|:-----|
| أ إ آ | ا | Alef variants → bare Alef |
| ى | ي | Alef Maksura → Ya |
| ة | ه | Taa Marbuta → Ha |
| ـ | *(removed)* | Tatweel (kashida) stripped |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_arabic::register_all(&mut factory);

let analyzer = factory.get_analyzer("arabic").unwrap();
```

## Installation

```toml
[dependencies]
pizza-analysis-arabic = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["arabic"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
