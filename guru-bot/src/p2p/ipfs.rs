use ipfs::{Ipfs, IpfsOptions, IpfsPath, TestTypes, UninitializedIpfs};
use std::process::exit;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

#[tokio::main]
pub async fn make_request() {
    // Initialize an in-memory repo and start a daemon.
    let opts = IpfsOptions::inmemory_with_generated_keys();
    let (ipfs, fut): (Ipfs<TestTypes>, _) = UninitializedIpfs::new(opts).start().await.unwrap();

    // Spawn the background task
    tokio::task::spawn(fut);

    // Restore the default bootstrappers to enable content discovery
    ipfs.restore_bootstrappers().await.unwrap();

    // Get the IPFS README
    let path = "/ipfs/QmQPeNsJPyVWPFDVHb77w8G42Fvo15z4bG2X8D2GhfbSXc/readme"
        .parse::<IpfsPath>()
        .unwrap();

    let stream = ipfs.cat_unixfs(path, None).await.unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    });

    tokio::pin!(stream);

    let mut stdout = tokio::io::stdout();

    loop {
        match stream.next().await {
            Some(Ok(bytes)) => {
                stdout.write_all(&bytes).await.unwrap();
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
            None => break,
        }
    }
}
