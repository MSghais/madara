use crate::client::EthereumClient;
use crate::l1_gas_price::gas_price_worker;
use crate::l1_messaging::sync;
use crate::state_update::state_update_worker;
use mc_mempool::{GasPriceProvider, Mempool};
use starknet_api::core::ChainId;
use std::sync::Arc;
use std::time::Duration;

use mc_db::MadaraBackend;

#[allow(clippy::too_many_arguments)]
pub async fn l1_sync_worker(
    backend: &MadaraBackend,
    eth_client: &EthereumClient,
    chain_id: ChainId,
    l1_gas_provider: GasPriceProvider,
    gas_price_sync_disabled: bool,
    gas_price_poll_ms: Duration,
    mempool: Arc<Mempool>,
    cancellation_token: tokio_util::sync::CancellationToken,
) -> anyhow::Result<()> {
    tokio::try_join!(
        state_update_worker(backend, eth_client, chain_id.clone(), cancellation_token.clone()),
        async {
            if !gas_price_sync_disabled {
                gas_price_worker(eth_client, l1_gas_provider, gas_price_poll_ms, cancellation_token.clone()).await?;
            }
            Ok(())
        },
        sync(backend, eth_client, &chain_id, mempool, cancellation_token.clone())
    )?;

    Ok(())
}
