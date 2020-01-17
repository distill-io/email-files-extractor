# Email Files Extractor 

An extractor to extract files from MHTML or Email files and store the files inside a specified path or in the same path.

### How to run the binary

1. Build the code

```rust
    cargo build --release
```

3. To run this binary, it accepts a parameter and a flag

```
    ./target/release/mail_extractor_binary [INFILE-PATH] [OUTFLE-PATH]
```