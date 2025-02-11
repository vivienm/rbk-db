use std::{future::Future, path::Path, sync::Arc};

use tokio::{fs::File, io::AsyncWriteExt, sync::Semaphore, task::JoinSet};

use super::table::{self, Table};

#[derive(Clone, Debug)]
pub struct Client {
    reqwest: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            reqwest: reqwest::Client::new(),
        }
    }

    async fn download_table<T, P>(&self, path: P, timestamp: u64) -> anyhow::Result<()>
    where
        T: Table,
        P: AsRef<Path>,
    {
        // To avoid consistency issues with untimestamped URLs and to avoid scraping the
        // main page for proper timestamps, the simplest solution is to use the
        // current timestamp.
        let url = format!(
            "https://cdn.rebrickable.com/media/downloads/{}?{}",
            T::FILENAME,
            timestamp
        );
        tracing::info!("downloading table {}", T::NAME);
        let mut response = self.reqwest.get(&url).send().await?.error_for_status()?;

        let mut file = File::create(&path).await?;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }
        file.flush().await?;

        Ok(())
    }

    pub fn download_tables<P>(&self, output_dir: P, timestamp: u64) -> DownloadHandler<P>
    where
        P: AsRef<Path>,
    {
        DownloadHandler {
            client: self,
            output_dir,
            max_concurrency: 4,
            timestamp,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
#[must_use]
pub struct DownloadHandler<'a, P> {
    client: &'a Client,
    output_dir: P,
    max_concurrency: usize,
    timestamp: u64,
}

impl<P> DownloadHandler<'_, P> {
    pub async fn execute(self) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let output_dir = self.output_dir.as_ref();
        let semaphore = Arc::new(Semaphore::new(self.max_concurrency));

        macro_rules! download_task {
            ($table:ty) => {
                download_task::<$table>(self.client, &semaphore, output_dir, self.timestamp)
            };
        }

        let mut set = JoinSet::new();
        set.spawn(download_task!(table::Inventories));
        set.spawn(download_task!(table::InventoryParts));
        set.spawn(download_task!(table::InventoryMinifigs));
        set.spawn(download_task!(table::InventorySets));
        set.spawn(download_task!(table::Parts));
        set.spawn(download_task!(table::PartCategories));
        set.spawn(download_task!(table::PartRelationships));
        set.spawn(download_task!(table::Elements));
        set.spawn(download_task!(table::Colors));
        set.spawn(download_task!(table::Minifigs));
        set.spawn(download_task!(table::Sets));
        set.spawn(download_task!(table::Themes));

        // Check for any errors.
        while let Some(res) = set.join_next().await {
            res??;
        }

        Ok(())
    }
}

fn download_task<T>(
    client: &Client,
    semaphore: &Arc<Semaphore>,
    output_dir: &Path,
    timestamp: u64,
) -> impl Future<Output = anyhow::Result<()>> + 'static
where
    T: Table,
{
    let client = client.clone();
    let semaphore = Arc::clone(semaphore);

    let mut output_path = output_dir.to_path_buf();
    output_path.push(T::FILENAME);

    async move {
        let _permit = semaphore.acquire().await.unwrap();
        client
            .download_table::<T, _>(&output_path, timestamp)
            .await?;
        Ok(())
    }
}
