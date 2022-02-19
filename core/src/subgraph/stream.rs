use crate::subgraph::inputs::IndexingInputs;
use graph::blockchain::block_stream::{BlockStream, BlockStreamMetrics, BufferedBlockStream};
use graph::blockchain::Blockchain;
use graph::prelude::{CheapClone, Error};
use std::sync::Arc;

const BUFFERED_BLOCK_STREAM_SIZE: usize = 100;
const BUFFERED_FIREHOSE_STREAM_SIZE: usize = 1;

pub async fn new_block_stream<C: Blockchain>(
    inputs: Arc<IndexingInputs<C>>,
    filter: C::TriggerFilter,
    block_stream_metrics: Arc<BlockStreamMetrics>,
) -> Result<Box<dyn BlockStream<C>>, Error> {
    let chain = inputs.chain.cheap_clone();
    let is_firehose = chain.is_firehose_supported();

    let buffer_size = match is_firehose {
        true => BUFFERED_FIREHOSE_STREAM_SIZE,
        false => BUFFERED_BLOCK_STREAM_SIZE,
    };

    let block_stream = match is_firehose {
        true => chain.new_firehose_block_stream(
            inputs.deployment.clone(),
            inputs.store.block_cursor(),
            inputs.start_blocks.clone(),
            Arc::new(filter.clone()),
            block_stream_metrics.clone(),
            inputs.unified_api_version.clone(),
        ),
        false => {
            let current_ptr = inputs.store.block_ptr();

            chain.new_polling_block_stream(
                inputs.deployment.clone(),
                inputs.start_blocks.clone(),
                current_ptr,
                Arc::new(filter.clone()),
                block_stream_metrics.clone(),
                inputs.unified_api_version.clone(),
            )
        }
    }
    .await?;

    Ok(BufferedBlockStream::spawn_from_stream(
        block_stream,
        buffer_size,
    ))
}
