use crate::subgraph::inputs::IndexingInputs;
use graph::blockchain::block_stream::{BlockStream, BlockStreamMetrics, BufferedBlockStream};
use graph::blockchain::Blockchain;
use graph::prelude::*;
use std::sync::Arc;

const BUFFERED_BLOCK_STREAM_SIZE: usize = 100;
const BUFFERED_FIREHOSE_STREAM_SIZE: usize = 1;

async fn new_block_stream<C: Blockchain>(
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

pub struct BlockStreamManager<C: Blockchain, F> {
    stream: Cancelable<Box<dyn BlockStream<C>>, F>,
}

impl<C, F> BlockStreamManager<C, F>
where
    C: Blockchain,
    F: Fn() -> CancelableError,
{
    pub async fn new(
        inputs: Arc<IndexingInputs<C>>,
        filter: C::TriggerFilter,
        block_stream_metrics: Arc<BlockStreamMetrics>,
    ) -> Result<(Self, CancelGuard), Error> {
        let block_stream_canceler = CancelGuard::new();

        let stream = new_block_stream(inputs, filter, block_stream_metrics)
            .await?
            .map_err(CancelableError::Error)
            .into_inner();

        let stream = stream.cancelable(&block_stream_canceler, || Err(CancelableError::Cancel));

        // expected struct `Box<dyn BlockStream<C, Item = Result<BlockStreamEvent<C>, graph::prelude::Error>>>`
        // found struct `Cancelable<graph::prelude::futures::stream::MapErr<Box<dyn BlockStream<C, Item = Result<BlockStreamEvent<C>, graph::prelude::Error>>>, fn(graph::prelude::Error) -> graph::prelude::CancelableError<graph::prelude::Error> {graph::prelude::CancelableError::<graph::prelude::Error>::Error}>, [closure@core/src/subgraph/stream.rs:71:49: 71:80]>`

        Ok((Self { stream }, block_stream_canceler))
    }
}
