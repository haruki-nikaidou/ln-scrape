use async_trait::async_trait;
use reqwest::{Error, Response, Url};

#[cfg(feature = "proxy")]
mod proxy;
mod user_agent;

#[cfg(feature = "proxy")]
pub use proxy::ProxyList;

pub use user_agent::UserAgentList;

#[async_trait]
pub trait RequestSenderTrait: Send + Sync {
    async fn req(&self, url: &Url) -> Result<reqwest::Response, reqwest::Error>;
}

pub struct RequestSender {
    user_agent_list: UserAgentList,
    #[cfg(feature = "proxy")]
    proxy_list: Option<ProxyList>,
    cookie: Option<String>
}

impl RequestSender {
    pub fn new() -> Self {
        RequestSender {
            user_agent_list: UserAgentList::new(),
            #[cfg(feature = "proxy")]
            proxy_list: None,
            cookie: None
        }
    }

    pub fn user_agent(self, user_agent_list: UserAgentList) -> Self {
        RequestSender {
            user_agent_list,
            #[cfg(feature = "proxy")]
            proxy_list: self.proxy_list,
            cookie: self.cookie
        }
    }

    #[cfg(feature = "proxy")]
    pub fn proxy(self, proxy_list: ProxyList) -> Self {
        RequestSender {
            user_agent_list: self.user_agent_list,
            proxy_list: Some(proxy_list),
            cookie: self.cookie
        }
    }

    pub fn cookie(self, cookie: String) -> Self {
        RequestSender {
            user_agent_list: self.user_agent_list,
            #[cfg(feature = "proxy")]
            proxy_list: self.proxy_list,
            cookie: Some(cookie)
        }
    }
}

#[async_trait]
impl RequestSenderTrait for RequestSender {
    async fn req(&self, url: &Url) -> Result<Response, Error> {
        #[allow(unused_mut)]
        let mut client = reqwest::Client::builder()
            .user_agent(self.user_agent_list.get_random())
            .gzip(true)
            .brotli(true)
            .zstd(true)
            .deflate(true);
        #[cfg(feature = "proxy")]
        if let Some(proxy) = &self.proxy_list {
            client = client.proxy(proxy.get_next_proxy().await);
        }
        let mut req = client.build()?.get(url.clone());
        if let Some(cookie) = &self.cookie {
            req = req.header("Cookie", cookie);
        }
        let res = req
            .header("Referer", url.domain().unwrap())
            .header("Accept-Language", "zh-CN,zh;q=0.9")
            .header("Accept", "*/*")
            .send().await?;
        Ok(res)
    }
}