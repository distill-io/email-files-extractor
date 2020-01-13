# Email Files Extractor 

An extractor to extract files from MHTML or Email files which is stored in Amazon S3 and store the files inside a specified path.

### How to run the binary

1. Build the code

```rust
    cargo build --release
```

2. After build is triggered a binary file will be created inside **./target/release/mht-extractor**, now set the environment variables for AWS configuration. Run in terminal.
```
    export bucket_name=[BUCKET_NAME]
    export region=[REGION]
    export access_key=[YOUR_AWS_ACCESS_KEY]
    export secret_key=[YOUR_AWS_SECRET_KEY]
```

3. To run this binary, it accepts a parameter and a flag

```
    ./target/release/email-extractor [ID] -o [PATH]
```