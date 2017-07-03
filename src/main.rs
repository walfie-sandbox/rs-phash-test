#[macro_use]
extern crate error_chain;

extern crate stream_dct;
extern crate image;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

use futures::{Future, IntoFuture, Stream};
use hyper_tls::HttpsConnector;
use phash::ImageHash;
use std::collections::HashSet;
use tokio_core::reactor::Core;

mod phash;
mod error;
mod bosses;

use bosses::{BOSSES, Boss, EXPECTED_PAIRS, Language};
use error::*;

type Client = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

#[derive(Debug, PartialEq)]
struct BossWithHash {
    boss: &'static Boss,
    hash: ImageHash,
}

quick_main!(|| -> Result<()> {
    let mut core = Core::new().chain_err(|| "failed to initialize core")?;
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(HttpsConnector::new(4, &handle).chain_err(|| "HTTPS error")?)
        .build(&handle);

    let bosses_future = futures::future::join_all(BOSSES.iter().map(move |boss| {
        let url = format!("{}:large", boss.url);
        get_and_hash(&client, &url).map(move |hash| BossWithHash { boss, hash })
    }));

    println!("Fetching bosses...");

    let bosses = core.run(bosses_future).chain_err(
        || "failed to get boss hashes",
    )?;

    println!("Comparing boss image hashes...");
    let mut results = HashSet::new();

    for b1 in bosses.iter() {
        for b2 in bosses.iter() {
            if b1.boss.language != b2.boss.language && b1.boss.level == b2.boss.level &&
                b1.hash == b2.hash
            {
                if b1.boss.language == Language::Ja {
                    results.insert((b1.boss.name, b2.boss.name));
                } else {
                    results.insert((b2.boss.name, b1.boss.name));
                }
            }
        }
    }

    use std::iter::FromIterator;
    let expected = HashSet::from_iter(EXPECTED_PAIRS.iter().cloned());

    assert_eq!(results, expected);

    Ok(())
});

fn get_and_hash(client: &Client, url: &str) -> Box<Future<Item = ImageHash, Error = Error>> {
    let url_string = url.to_string();

    let result = url.parse()
        .chain_err(|| "invalid URL")
        .map(|u| client.get(u).then(|r| r.chain_err(|| "HTTP error")))
        .into_future()
        .flatten()
        .and_then(|resp| {
            println!("got response");

            resp.body().concat2().then(|r| {
                println!("concat");

                r.chain_err(|| "failed to get body")
            })
        })
        .and_then(|bytes| hash(&bytes).into_future())
        .then(move |r| {
            println!("Fetched url {}", url_string);
            r.chain_err(|| format!("failed to get URL: `{}`", url_string))
        });

    Box::new(result)
}

fn hash(bytes: &[u8]) -> Result<ImageHash> {
    use image::GenericImage;

    let mut img = image::load_from_memory(bytes).chain_err(
        || "failed to load bytes",
    )?;
    let (w, h) = img.dimensions();
    img = img.crop(0, 0, w, h * 3 / 4);

    Ok(phash::ImageHash::new(&img))
}
