use reqwest::Proxy;
use tokio::sync::Mutex;

/// **非常邪恶的东西，不要滥用**
///
/// 有的站会限制单个IP的访问频率，这东西可以用来绕过这个限制。注意别把人家网站打死了。
pub struct ProxyList {
    proxy_list: Vec<Proxy>,
    current: Mutex<usize>
}

impl ProxyList {
    pub(crate) async fn get_next_proxy(&self) -> Proxy {
        let mut current = self.current.lock().await;
        let current_proxy;
        if *current >= self.proxy_list.len() - 1{
            *current = 0;
            current_proxy = &self.proxy_list[0];
        } else {
            *current += 1;
            current_proxy = &self.proxy_list[*current];
        }
        current_proxy.clone()
    }
    fn new(list: Vec<String>) -> Self {
        let proxy_list: Vec<Proxy> = list
            .iter()
            .map(|x| Proxy::all(x))
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        ProxyList {
            proxy_list,
            current: Mutex::new(0)
        }
    }
}