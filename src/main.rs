#[macro_use]
extern crate error_chain;

extern crate image;
extern crate img_hash;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

use futures::{Future, IntoFuture, Stream};
use hyper_tls::HttpsConnector;
use img_hash::ImageHash;
use tokio_core::reactor::Core;

mod error;

use error::*;

type Client = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

quick_main!(|| -> Result<()> {
    let mut core = Core::new().chain_err(|| "failed to initialize core")?;
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(HttpsConnector::new(4, &handle).chain_err(|| "HTTPS error")?)
        .build(&handle);

    let compare = Boss::new(
        "Lv60 Yggdrasil Omega",
        "https://pbs.twimg.com/media/CT6cDD3UkAEnP8Y.jpg:large",
        "https://pbs.twimg.com/media/CfuZgxLUkAArdGe.jpg:large",
    ).compare(&client);

    assert_eq!(core.run(compare).chain_err(|| "failed to compare")?, ());

    Ok(())
});

struct Boss {
    name: &'static str,
    url1: &'static str,
    url2: &'static str,
}

impl Boss {
    fn new(name: &'static str, url1: &'static str, url2: &'static str) -> Self {
        Boss { name, url1, url2 }
    }

    fn compare(&self, client: &Client) -> Box<Future<Item = (), Error = Error>> {
        let name = ::std::borrow::Cow::Borrowed(self.name);

        let result =
            compute_dist(client, self.url1, self.url2).and_then(move |dist| if dist == 0 {
                Ok(())
            } else {
                bail!(format!("{} distance: {}", name, dist))
            });

        Box::new(result)
    }
}

fn compute_dist(
    client: &Client,
    url1: &str,
    url2: &str,
) -> Box<Future<Item = usize, Error = Error>> {
    let hash1 = get_and_hash(client, url1);
    let hash2 = get_and_hash(client, url2);

    Box::new(hash1.join(hash2).map(|(h1, h2)| h1.dist(&h2)))
}


fn get_and_hash(client: &Client, url: &str) -> Box<Future<Item = ImageHash, Error = Error>> {
    let url_string = url.to_string();

    let result = url.parse()
        .chain_err(|| "invalid URL")
        .map(|u| client.get(u).then(|r| r.chain_err(|| "HTTP error")))
        .into_future()
        .flatten()
        .and_then(|resp| {
            resp.body().concat2().then(|r| {
                r.chain_err(|| "failed to get body")
            })
        })
        .and_then(|bytes| hash(&bytes).into_future())
        .then(move |r| {
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
    img.crop(0, 0, w, h * 3 / 4);

    Ok(img_hash::ImageHash::hash(
        &img,
        8,
        img_hash::HashType::Gradient,
    ))
}
