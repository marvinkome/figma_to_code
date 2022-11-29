use reqwest::{header, Client};

#[derive(Debug)]
pub struct RaxiosClient {
    client: Client,
}

#[allow(dead_code)]
impl RaxiosClient {
    pub fn new(headers: Option<header::HeaderMap>) -> Self {
        let mut client_headers = header::HeaderMap::new();

        // add default headers
        client_headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());
        client_headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());

        // add args headers
        match headers {
            Some(headers) => {
                for (key, value) in headers.iter() {
                    client_headers.insert(key, value.to_owned());
                }
            }
            None => (),
        };
        
        // build client
        let client = Client::builder()
            .default_headers(client_headers)
            .build()
            .expect("Failed to build client. Please check your configurations");
        

        Self { 
            client,
        }
    }

    pub async fn get(
        &self,
        url: &str,
        headers: Option<header::HeaderMap>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request_builder = self.client.get(url);

        let request_builder = match headers {
            None => request_builder,
            Some(headers) => request_builder.headers(headers),
        };

        let response = request_builder.send().await?;
        return Ok(response);
    }

    // pub async fn post<T: for<'de> Deserialize<'de>>(
    //     &self,
    //     url: &str,
    //     body: Option<HashMap<String, String>>,
    //     headers: Option<header::HeaderMap>,
    // ) -> Result<T, reqwest::Error> {
    //     let client = Client::new();
    //     let request_builder = client.get(url);

    //     let request_builder = match body {
    //         None => request_builder,
    //         Some(body) => request_builder.json(&body),
    //     };

    //     let request_builder = match &self.headers {
    //         None => request_builder,
    //         Some(headers) => request_builder.headers(headers.clone()),
    //     };

    //     let request_builder = match headers {
    //         None => request_builder,
    //         Some(headers) => request_builder.headers(headers),
    //     };

    //     let response = request_builder.send().await?;
    //     response.json::<T>().await
    // }
}
