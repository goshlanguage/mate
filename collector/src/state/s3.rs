use async_std::task::block_on;
use log::{error, info};
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::env;

pub struct S3 {
    bucket_name: String,
    credentials: Credentials,
    region: Region,
}

impl S3 {
    pub fn new(name: String, proto: String, region: String) -> S3 {
        let s3_endpoint = match env::var("BUCKET_HOST") {
            Ok(val) => val,
            Err(e) => panic!(
                "S3 configured but no endpoint exists in the environment. {}",
                e
            ),
        };

        let endpoint = format!("{}://{}", proto, s3_endpoint);
        info!("configured new s3 endpoint {}", endpoint);

        let region = Region::Custom { region, endpoint };
        let credentials = Credentials::default().unwrap();

        S3 {
            bucket_name: name,
            credentials,
            region,
        }
    }

    pub fn default() -> S3 {
        S3 {
            bucket_name: "".to_string(),
            credentials: Credentials::new(Some(""), Some(""), None, None, None).unwrap(),
            region: Region::UsEast1,
        }
    }

    /// Bucket exists to construct the bucket for use, so that it doesn't need to be saved on a struct
    /// directly. Mostly done because unsure about safe usage of None type for this
    fn bucket(&self) -> Bucket {
        let bucket_name = self.bucket_name.as_str();
        let bucket_region = &self.region;
        let bucket_creds = &self.credentials;

        Bucket::new_with_path_style(
            bucket_name,
            bucket_region.to_owned(),
            bucket_creds.to_owned(),
        )
        .unwrap()
    }

    pub fn save(&self, filepath: String, data: String) {
        let path = filepath;
        let bucket = self.bucket();

        block_on(async {
            let (_, code) = bucket
                .put_object_blocking(path.clone(), data.as_bytes())
                .unwrap();
            info!("path: {}\tcode: {}", path, code);
            // TODO
            // Traige why this error isn't printing with -vv
            if !code == 200 {
                error!("Failed to put data in the bucket: {}\tcode: {}", path, code);
            }
        });
    }
}
