use super::apay::*;
use crate::service::prelude::*;
use md5::Digest as Md5Digest;
use reqwest::Client;

/// 微信支付客户端 (V2 XML接口)
pub struct WechatPayClient {
    client: Client,
    app_id: String,
    mch_id: String,
    api_key: String,
    /// 证书路径 (用于退款等需要证书的接口)
    cert_path: Option<String>,
    cert_password: Option<String>,
}

impl WechatPayClient {
    pub fn new(app_id: String, mch_id: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            app_id,
            mch_id,
            api_key,
            cert_path: None,
            cert_password: None,
        }
    }

    pub fn with_cert(mut self, cert_path: String, cert_password: Option<String>) -> Self {
        self.cert_path = Some(cert_path);
        self.cert_password = cert_password;
        self
    }

    /// 生成随机字符串
    fn generate_nonce_str() -> String {
        use rand::RngExt;
        let mut rng = rand::rng();
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        (0..32)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// 计算签名 (MD5)
    fn sign(&self, params: &serde_json::Map<String, serde_json::Value>) -> String {
        let mut keys: Vec<&String> = params.keys().collect();
        keys.sort();

        let mut sign_str = String::new();
        for key in keys {
            if key == "sign" {
                continue;
            }
            let value = params.get(key).and_then(|v| v.as_str()).unwrap_or("");
            if !value.is_empty() {
                if !sign_str.is_empty() {
                    sign_str.push('&');
                }
                sign_str.push_str(&format!("{}={}", key, value));
            }
        }
        sign_str.push_str(&format!("&key={}", self.api_key));

        let digest = md5::Md5::digest(sign_str.as_bytes());
        hex::encode(digest)
    }

    /// 验证回调签名
    pub fn verify_sign(&self, params: &serde_json::Map<String, serde_json::Value>) -> bool {
        let received_sign = params
            .get("sign")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let calculated_sign = self.sign(params);
        calculated_sign.eq_ignore_ascii_case(received_sign)
    }

    /// 统一下单
    #[warn(clippy::too_many_arguments)]
    pub async fn unified_order(
        &self,
        body: &str,
        out_trade_no: &str,
        total_fee: i64,
        spbill_create_ip: &str,
        notify_url: &str,
        trade_type: &TradeType,
        openid: Option<&str>,
        attach: Option<&str>,
        time_expire: Option<&str>,
    ) -> Result<UnifiedOrderResponse> {
        let nonce_str = Self::generate_nonce_str();
        let mut params = serde_json::Map::new();
        params.insert("appid".to_string(), serde_json::Value::String(self.app_id.clone()));
        params.insert("mch_id".to_string(), serde_json::Value::String(self.mch_id.clone()));
        params.insert("nonce_str".to_string(), serde_json::Value::String(nonce_str.clone()));
        params.insert("sign_type".to_string(), serde_json::Value::String("MD5".to_string()));
        params.insert("body".to_string(), serde_json::Value::String(body.to_string()));
        params.insert("out_trade_no".to_string(), serde_json::Value::String(out_trade_no.to_string()));
        params.insert("total_fee".to_string(), serde_json::Value::String(total_fee.to_string()));
        params.insert("spbill_create_ip".to_string(), serde_json::Value::String(spbill_create_ip.to_string()));
        params.insert("notify_url".to_string(), serde_json::Value::String(notify_url.to_string()));
        params.insert("trade_type".to_string(), serde_json::Value::String(trade_type.as_str().to_string()));

        if let Some(oid) = openid {
            params.insert("openid".to_string(), serde_json::Value::String(oid.to_string()));
        }
        if let Some(att) = attach {
            params.insert("attach".to_string(), serde_json::Value::String(att.to_string()));
        }
        if let Some(te) = time_expire {
            params.insert("time_expire".to_string(), serde_json::Value::String(te.to_string()));
        }

        let sign = self.sign(&params);
        params.insert("sign".to_string(), serde_json::Value::String(sign));

        // 构建XML
        let xml = self.map_to_xml(&params);

        let response = self
            .client
            .post("https://api.mch.weixin.qq.com/pay/unifiedorder")
            .header("Content-Type", "application/xml; charset=utf-8")
            .body(xml)
            .send()
            .await?;

        let body = response.text().await?;
        let result: UnifiedOrderResponse = serde_xml_rs::from_str(&body)
            .map_err(|e| format!("解析微信支付响应失败: {}", e))?;

        Ok(result)
    }

    /// 查询订单
    pub async fn order_query(
        &self,
        out_trade_no: Option<&str>,
        transaction_id: Option<&str>,
    ) -> Result<OrderQueryResponse> {
        let nonce_str = Self::generate_nonce_str();
        let mut params = serde_json::Map::new();
        params.insert("appid".to_string(), serde_json::Value::String(self.app_id.clone()));
        params.insert("mch_id".to_string(), serde_json::Value::String(self.mch_id.clone()));
        params.insert("nonce_str".to_string(), serde_json::Value::String(nonce_str));
        params.insert("sign_type".to_string(), serde_json::Value::String("MD5".to_string()));

        if let Some(no) = out_trade_no {
            params.insert("out_trade_no".to_string(), serde_json::Value::String(no.to_string()));
        }
        if let Some(txid) = transaction_id {
            params.insert("transaction_id".to_string(), serde_json::Value::String(txid.to_string()));
        }

        let sign = self.sign(&params);
        params.insert("sign".to_string(), serde_json::Value::String(sign));

        let xml = self.map_to_xml(&params);

        let response = self
            .client
            .post("https://api.mch.weixin.qq.com/pay/orderquery")
            .header("Content-Type", "application/xml; charset=utf-8")
            .body(xml)
            .send()
            .await?;

        let body = response.text().await?;
        let result: OrderQueryResponse = serde_xml_rs::from_str(&body)
            .map_err(|e| format!("解析查询订单响应失败: {}", e))?;

        Ok(result)
    }

    /// 关闭订单
    pub async fn close_order(&self, out_trade_no: &str) -> Result<CloseOrderResponse> {
        let nonce_str = Self::generate_nonce_str();
        let mut params = serde_json::Map::new();
        params.insert("appid".to_string(), serde_json::Value::String(self.app_id.clone()));
        params.insert("mch_id".to_string(), serde_json::Value::String(self.mch_id.clone()));
        params.insert("nonce_str".to_string(), serde_json::Value::String(nonce_str));
        params.insert("sign_type".to_string(), serde_json::Value::String("MD5".to_string()));
        params.insert("out_trade_no".to_string(), serde_json::Value::String(out_trade_no.to_string()));

        let sign = self.sign(&params);
        params.insert("sign".to_string(), serde_json::Value::String(sign));

        let xml = self.map_to_xml(&params);

        let response = self
            .client
            .post("https://api.mch.weixin.qq.com/pay/closeorder")
            .header("Content-Type", "application/xml; charset=utf-8")
            .body(xml)
            .send()
            .await?;

        let body = response.text().await?;
        let result: CloseOrderResponse = serde_xml_rs::from_str(&body)
            .map_err(|e| format!("解析关闭订单响应失败: {}", e))?;

        Ok(result)
    }

    /// 申请退款
    pub async fn refund(
        &self,
        out_trade_no: &str,
        out_refund_no: &str,
        total_fee: i64,
        refund_fee: i64,
        refund_desc: Option<&str>,
        notify_url: Option<&str>,
    ) -> Result<RefundResponse> {
        let nonce_str = Self::generate_nonce_str();
        let mut params = serde_json::Map::new();
        params.insert("appid".to_string(), serde_json::Value::String(self.app_id.clone()));
        params.insert("mch_id".to_string(), serde_json::Value::String(self.mch_id.clone()));
        params.insert("nonce_str".to_string(), serde_json::Value::String(nonce_str));
        params.insert("sign_type".to_string(), serde_json::Value::String("MD5".to_string()));
        params.insert("out_trade_no".to_string(), serde_json::Value::String(out_trade_no.to_string()));
        params.insert("out_refund_no".to_string(), serde_json::Value::String(out_refund_no.to_string()));
        params.insert("total_fee".to_string(), serde_json::Value::String(total_fee.to_string()));
        params.insert("refund_fee".to_string(), serde_json::Value::String(refund_fee.to_string()));

        if let Some(desc) = refund_desc {
            params.insert("refund_desc".to_string(), serde_json::Value::String(desc.to_string()));
        }
        if let Some(url) = notify_url {
            params.insert("notify_url".to_string(), serde_json::Value::String(url.to_string()));
        }

        let sign = self.sign(&params);
        params.insert("sign".to_string(), serde_json::Value::String(sign));

        let xml = self.map_to_xml(&params);

        // 退款接口需要双向证书，这里使用普通请求
        // 生产环境需要配置证书
        let response = self
            .client
            .post("https://api.mch.weixin.qq.com/secapi/pay/refund")
            .header("Content-Type", "application/xml; charset=utf-8")
            .body(xml)
            .send()
            .await?;

        let body = response.text().await?;
        let result: RefundResponse = serde_xml_rs::from_str(&body)
            .map_err(|e| format!("解析退款响应失败: {}", e))?;

        Ok(result)
    }

    /// 生成 JSAPI 支付参数 (给前端调起支付)
    pub fn jsapi_pay_params(&self, prepay_id: &str) -> JsapiPayParams {
        let nonce_str = Self::generate_nonce_str();
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let package = format!("prepay_id={}", prepay_id);

        let mut params = serde_json::Map::new();
        params.insert("appId".to_string(), serde_json::Value::String(self.app_id.clone()));
        params.insert("timeStamp".to_string(), serde_json::Value::String(timestamp.clone()));
        params.insert("nonceStr".to_string(), serde_json::Value::String(nonce_str.clone()));
        params.insert("package".to_string(), serde_json::Value::String(package.clone()));
        params.insert("signType".to_string(), serde_json::Value::String("MD5".to_string()));

        let sign = self.sign(&params);

        JsapiPayParams {
            appid: self.app_id.clone(),
            partnerid: self.mch_id.clone(),
            prepayid: prepay_id.to_string(),
            package,
            noncestr: nonce_str,
            timestamp,
            sign,
        }
    }

    /// 解析支付回调通知
    pub fn parse_pay_notify(&self, xml: &str) -> Result<PayNotify> {
        let notify: PayNotify = serde_xml_rs::from_str(xml)
            .map_err(|e| format!("解析支付回调XML失败: {}", e))?;
        Ok(notify)
    }

    /// 解析退款回调通知
    pub fn parse_refund_notify(&self, xml: &str) -> Result<RefundNotify> {
        // 退款回调的加密信息需要解密，这里先做基本解析
        let notify: RefundNotify = serde_xml_rs::from_str(xml)
            .map_err(|e| format!("解析退款回调XML失败: {}", e))?;
        Ok(notify)
    }

    /// 生成成功响应XML
    pub fn success_response() -> String {
        let resp = NotifyResponse::success();
        serde_xml_rs::to_string(&resp).unwrap_or_else(|_| "<xml><return_code>SUCCESS</return_code></xml>".to_string())
    }

    /// 生成失败响应XML
    pub fn fail_response(msg: &str) -> String {
        let resp = NotifyResponse::fail(msg);
        serde_xml_rs::to_string(&resp).unwrap_or_else(|_| format!("<xml><return_code>FAIL</return_code><return_msg>{}</return_msg></xml>", msg))
    }

    /// 将 Map 转换为 XML 字符串
    fn map_to_xml(&self, params: &serde_json::Map<String, serde_json::Value>) -> String {
        let mut xml = String::from("<xml>");
        for (key, value) in params {
            let v = value.as_str().unwrap_or("");
            xml.push_str(&format!("<{key}>{v}</{key}>", key = key, v = v));
        }
        xml.push_str("</xml>");
        xml
    }
}
