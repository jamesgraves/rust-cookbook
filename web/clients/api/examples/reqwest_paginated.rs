use reqwest::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    // dependencies: Vec<Dependency>,
    meta: Meta,
    versions: Vec<Version>,
}

#[derive(Deserialize, Debug)]
struct Dependency {
    // crate_id: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

#[derive(Deserialize, Debug)]
struct Version {
    // In the JSON returned from crates.io, there is a field named 'crate' which is the name
    // of the crate dependency. But that is a keyword in Rust, so the struct member is renamed.
    #[serde(rename = "crate")]
    crate_name: String,
    num: String,
}

struct ReverseDependencies {
    crate_id: String,
    versions: <Vec<Version> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        let rq_client =  reqwest::blocking::Client::builder()
            .user_agent("Rust Cookbook (https://jamesgraves.github.io/rust-cookbook/)")
            .build()?;
        Ok(ReverseDependencies {
               crate_id: crate_id.to_owned(),
               versions: vec![].into_iter(),
               client: rq_client,
               page: 0,
               per_page: 10,
               total: 0,
           })
    }

    fn try_next(&mut self) -> Result<Option<Version>> {
        if let Some(dep) = self.versions.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
                          self.crate_id,
                          self.page,
                          self.per_page);

        let response = self.client.get(&url).send()?;
        let decoded_response = response.json::<ApiResponse>()?;
        self.versions = decoded_response.versions.into_iter();
        self.total = decoded_response.meta.total;
        Ok(self.versions.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Version>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn main() -> Result<()> {
    let crate_name = "trycmd".to_string();
    println!("reverse dependencies of: {}", &crate_name);
    for dep in ReverseDependencies::of(&crate_name)? {
        let this_dep = dep?;
        println!("    {} {}", this_dep.crate_name, this_dep.num);
    }
    Ok(())
}
