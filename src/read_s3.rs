use s3::bucket::Bucket;
use s3::region::Region;
use s3::credentials::Credentials;
use std::str::FromStr;
use std::env;
use std::option::Option;

pub fn init(filename: &str) -> Vec<u8> {
    let bucket_name = match env::var("bucket_name") {
        Ok(bucket_name) => bucket_name,
        Err(err) => panic!(err),
    };
    let region: String = match env::var("region") {
        Ok(region) => region,
        Err(err) => panic!(err)
    };
    let access_key: Option<String> = Some(
        match env::var("access_key") {
            Ok(access_key) => access_key,
            Err(err) => panic!(err)
        }
    );
    let secret_key: Option<String> = Some(
        match env::var("secret_key") {
            Ok(secret_key) => secret_key,
            Err(err) => panic!(err),
        }
    );
    let credentials = Credentials::new(access_key, secret_key, None, None);

    let bucket = Bucket::new(&bucket_name, Region::from_str(&region).unwrap(), credentials).unwrap();
    let (data, _) = bucket.get_object(&("/".to_owned() + filename +".mht.gz")).unwrap();
    data
}
