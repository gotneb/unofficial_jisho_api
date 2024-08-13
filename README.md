![Jisho's logo](https://raw.githubusercontent.com/mistval/unofficial-jisho-api/master/logo.png)

# unofficial jisho api
I'm excited about Rust and really enjoying programming on it.
That's also my first time uploading and contributing to the community.

If you have any improvements please submit a pull request. I'll be glad learning and getting in touch with other developers.

That was inspired and an attempt to rewrite [Mistval's unofficial jisho api.](https://github.com/mistval/unofficial-jisho-api)

## Example
```rust
let result = JishoAPI::search_kanji("語".into()).unwrap();

println!("Grade: {}", result.taught);
println!("Level: {:?}", result.jlpt_level);
println!("Stroke numbers: {}", result.stroke_count);
println!("Meaning: {}", result.meaning);
println!("Kunyomi: {:?}", result.kunyomi);
println!("Onyomi: {:?}", result.onyomi);
println!("Kanji parts: {:?}", result.parts);

println!("Kanji: {:?}", result.kunyomi_examples[0].kanji);
println!("Hiragana: {:?}", result.kunyomi_examples[0].hiragana);
println!("Meaning: {:?}", result.kunyomi_examples[0].meaning);
```

## Permission
Permission to scrape granted by Jisho's admin Kimtaro:
```
https://jisho.org/forum/54fefc1f6e73340b1f160000-is-there-any-kind-of-search-api
```
