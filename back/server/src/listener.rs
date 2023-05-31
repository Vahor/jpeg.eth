use std::sync::Arc;

use anyhow::Result;
use ethers::contract::{abigen, Contract};
use ethers::prelude::ValueOrArray;
use ethers::providers::{Provider, StreamExt, Ws};
use log::info;

use crate::db::{assign_image, get_random_unassigned_image, Pool};
use crate::env_helpers::cast_required_env_var;

abigen!(
    ERC721,
    r#"[
        event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)
    ]"#
);

pub async fn start_listener(pool: Pool) -> Result<()> {
    let host = cast_required_env_var::<String>("WSS_URL");

    let client: Provider<Ws> = Provider::<Ws>::connect(host).await?;
    let client = Arc::new(client);

    let event =
        Contract::event_of_type::<TransferFilter>(client).address(ValueOrArray::Array(vec![
            "0xB0377814cAC67bece4CD3a5C5319BE2368Ce21df"
                .parse()
                .unwrap(),
        ]));

    let mut stream = event.subscribe_with_meta().await?;

    info!("Listening for events...");

    while let Some(res) = stream.next().await {
        let (event, _) = res?;
        let to = format!("{:#x}", event.to);
        let token_id = event.token_id.to_string();

        println!("New transfer: to: {to}, token_id: {token_id}",);

        // save to db
        let conn = pool.get()?;
        let random_id = get_random_unassigned_image(&conn)?;
        println!("Assigned image {} to {} with nft id {}", &random_id, &to, &token_id);
        assign_image(&conn, token_id, random_id)?;
    }

    Ok(())
}
