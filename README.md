# pizza-analysis-arabic

Arabic language analysis with diacritics removal, letter form normalization, light stemming, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `arabic_normalization` | Token Filter | Removes diacritics (tashkeel), tatweel, normalizes Alef/Yeh/Teh Marbuta forms |
| `arabic_stem` | Token Filter | Light Arabic stemmer — removes definite articles and common affixes |
| `arabic_stop` | Token Filter | Arabic stop words filter (119 words) |
| `arabic` | Analyzer | Full pipeline: lowercase → normalization → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "arabic"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["arabic_normalization", "arabic_stem", "arabic_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
