use apibara_core::starknet::v1alpha2::{Block, Filter, HeaderFilter};
use apibara_sdk::{ClientBuilder, Configuration, Uri};
use futures_util::{StreamExt, TryStreamExt};

// #[tokio::test]
#[ignore]
async fn test_apibara_high_level_api() -> Result<(), Box<dyn std::error::Error>> {
    let (stream, configuration_handle) = ClientBuilder::<Filter, Block>::default()
        .with_bearer_token("my_auth_token".into())
        .connect(Uri::from_static("https://goerli.starknet.a5a.ch"))
        .await?;

    configuration_handle
        .send(
            Configuration::<Filter>::default()
                .with_starting_block(800_000)
                .with_filter(|mut filter| filter.with_header(HeaderFilter::new()).build()),
        )
        .await?;

    let mut stream = stream.take(2);
    while let Some(response) = stream.try_next().await? {
        println!("Response: {:?}", response);
    }

    Ok(())
}
