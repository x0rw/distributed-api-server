use std::collections::HashMap;

pub struct MinreqHttpClient;

impl MinreqHttpClient {
    pub fn new() -> Self {
        MinreqHttpClient {}
    }
}

#[async_trait]
impl HttpClient for MinreqHttpClient {
    async fn get(
        &self,
        url: &str,
        headers: Option<&HashMap<String, String>>,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut request = Minreq::get(url);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        let response = request.send()?;
        let status = response.status_code as u16;
        let response_headers = response
            .headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        let body = response.as_bytes().to_vec();

        Ok(Response {
            status,
            headers: response_headers,
            body,
        })
    }

    async fn post(
        &self,
        url: &str,
        headers: Option<&HashMap<String, String>>,
        body: Option<&[u8]>,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut request = Minreq::post(url);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        if let Some(body) = body {
            request = request.body(body.to_vec());
        }
        let response = request.send()?;
        let status = response.status_code as u16;
        let response_headers = response
            .headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        let body = response.as_bytes().to_vec();

        Ok(Response {
            status,
            headers: response_headers,
            body,
        })
    }

    async fn put(
        &self,
        url: &str,
        headers: Option<&HashMap<String, String>>,
        body: Option<&[u8]>,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut request = Minreq::put(url);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        if let Some(body) = body {
            request = request.body(body.to_vec());
        }
        let response = request.send()?;
        let status = response.status_code as u16;
        let response_headers = response
            .headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        let body = response.as_bytes().to_vec();

        Ok(Response {
            status,
            headers: response_headers,
            body,
        })
    }

    async fn delete(
        &self,
        url: &str,
        headers: Option<&HashMap<String, String>>,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut request = Minreq::delete(url);
        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        let response = request.send()?;
        let status = response.status_code as u16;
        let response_headers = response
            .headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        let body = response.as_bytes().to_vec();

        Ok(Response {
            status,
            headers: response_headers,
            body,
        })
    }
}
