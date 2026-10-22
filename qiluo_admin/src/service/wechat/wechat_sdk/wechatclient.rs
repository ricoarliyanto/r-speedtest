use super::material::amaterial::*;
use super::menu::amenu::*; 
use crate::service::prelude::*;
use reqwest::Client;

pub struct WeChatClient {
    client: Client,
    app_id: String,
    app_secret: String,
}

impl WeChatClient {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            client: Client::new(),
            app_id,
            app_secret,
        }
    }

    pub async fn get_access_token(&self) -> Result<String> {
        let cache = CacheManager::instance().await;
        if let Ok(token) = cache.get_string(&self.app_id).await {
            return Ok(token);
        }
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
            self.app_id, self.app_secret
        );
        let response = self.client.get(&url).send().await?;
        let token_response: AccessTokenResponse = response.json().await?;
        let cache = CacheManager::instance().await;
        let _ = cache
            .set_string_ex(
                &self.app_id,
                &token_response.access_token,
                7200,
            )
            .await;

        Ok(token_response.access_token)
    }

    pub async fn create_menu(&self, menu: &CustomMenu) -> Result<String> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/menu/create?access_token={}",
            access_token
        );

        let response = self.client.post(&url).json(menu).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok("菜单创建成功".to_string())
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("菜单创建失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }

    pub async fn delete_menu(&self) -> Result<String> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/menu/delete?access_token={}",
            access_token
        );

        let response = self.client.get(&url).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok("菜单删除成功".to_string())
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("菜单删除失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }

    pub async fn get_menu(&self) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/menu/get?access_token={}",
            access_token
        );

        let response = self.client.get(&url).send().await?;
        let result: serde_json::Value = response.json().await?;

        Ok(result)
    }

    /// 发送模板消息
    pub async fn send_template_message(
        &self,
        openid: &str,
        template_id: &str,
        data: &serde_json::Value,
        url: Option<&str>,
    ) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let api_url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/template/send?access_token={}",
            access_token
        );

        let mut body = serde_json::json!({
            "touser": openid,
            "template_id": template_id,
            "data": data,
        });

        if let Some(u) = url {
            body["url"] = serde_json::Value::String(u.to_string());
        }

        let response = self.client.post(&api_url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        Ok(result)
    }

    /// 获取模板列表
    pub async fn get_template_list(&self) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/template/get_all_private_template?access_token={}",
            access_token
        );

        let response = self.client.get(&url).send().await?;
        let result: serde_json::Value = response.json().await?;

        Ok(result)
    }

    /// 删除模板
    pub async fn delete_template(&self, template_id: &str) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/template/del_private_template?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "template_id": template_id,
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        Ok(result)
    }

    /// 设置所属行业
    pub async fn set_industry(&self, industry_id1: &str, industry_id2: &str) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/template/api_set_industry?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "industry_id1": industry_id1,
            "industry_id2": industry_id2,
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        Ok(result)
    }

    /// 获取所属行业
    pub async fn get_industry(&self) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/template/get_industry?access_token={}",
            access_token
        );

        let response = self.client.get(&url).send().await?;
        let result: serde_json::Value = response.json().await?;

        Ok(result)
    }

    /// 发送客服消息（文本）
    pub async fn send_custom_text_message(&self, openid: &str, content: &str) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "touser": openid,
            "msgtype": "text",
            "text": {
                "content": content
            }
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok(result)
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("客服消息发送失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }

    /// 发送客服消息（图片）
    pub async fn send_custom_image_message(&self, openid: &str, media_id: &str) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "touser": openid,
            "msgtype": "image",
            "image": {
                "media_id": media_id
            }
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok(result)
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("客服消息发送失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }

    /// 发送客服消息（图文链接）
    pub async fn send_custom_link_message(
        &self,
        openid: &str,
        title: &str,
        description: &str,
        url: &str,
        thumb_url: &str,
    ) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let api_url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "touser": openid,
            "msgtype": "link",
            "link": {
                "title": title,
                "description": description,
                "url": url,
                "thumb_url": thumb_url
            }
        });

        let response = self.client.post(&api_url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok(result)
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("客服消息发送失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }

    // ==================== 素材管理接口 ====================

    /// 上传临时素材（文件路径方式）
    pub async fn upload_temp_media(
        &self,
        media_type: &str,
        file_path: &str,
        file_name: &str,
    ) -> Result<UploadTempMediaResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/media/upload?access_token={}&type={}",
            access_token, media_type
        );

        let file_bytes = tokio::fs::read(file_path).await
            .map_err(|e| format!("读取文件失败: {}", e))?;
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string())
            .mime_str(match media_type {
                "image" => "image/jpeg",
                "voice" => "audio/mpeg",
                "video" => "video/mp4",
                _ => "application/octet-stream",
            })
            .unwrap_or_else(|_| reqwest::multipart::Part::bytes(vec![]).file_name("file"));
        let form = reqwest::multipart::Form::new().part("media", part);

        let response = self.client.post(&url).multipart(form).send().await?;
        let result: UploadTempMediaResponse = response.json().await?;
        Ok(result)
    }

    /// 上传临时素材（字节流方式）
    pub async fn upload_temp_media_bytes(
        &self,
        media_type: &str,
        file_bytes: Vec<u8>,
        file_name: &str,
    ) -> Result<UploadTempMediaResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/media/upload?access_token={}&type={}",
            access_token, media_type
        );

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("media", part);

        let response = self.client.post(&url).multipart(form).send().await?;
        let result: UploadTempMediaResponse = response.json().await?;
        Ok(result)
    }

    /// 获取临时素材（下载）
    pub async fn get_temp_media(&self, media_id: &str) -> Result<Vec<u8>> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/media/get?access_token={}&media_id={}",
            access_token, media_id
        );

        let response = self.client.get(&url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// 上传永久素材（图片/语音/缩略图）
    pub async fn upload_permanent_media(
        &self,
        media_type: &str,
        file_path: &str,
        file_name: &str,
    ) -> Result<UploadPermanentMediaResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/add_material?access_token={}&type={}",
            access_token, media_type
        );

        let file_bytes = tokio::fs::read(file_path).await
            .map_err(|e| format!("读取文件失败: {}", e))?;
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("media", part);

        let response = self.client.post(&url).multipart(form).send().await?;
        let result: UploadPermanentMediaResponse = response.json().await?;
        Ok(result)
    }

    /// 上传永久视频素材
    pub async fn upload_permanent_video(
        &self,
        file_path: &str,
        file_name: &str,
        title: &str,
        introduction: &str,
    ) -> Result<UploadPermanentMediaResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/add_material?access_token={}&type=video",
            access_token
        );

        let file_bytes = tokio::fs::read(file_path).await
            .map_err(|e| format!("读取文件失败: {}", e))?;
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new()
            .part("media", part)
            .text("description", serde_json::json!({
                "title": title,
                "introduction": introduction
            }).to_string());

        let response = self.client.post(&url).multipart(form).send().await?;
        let result: UploadPermanentMediaResponse = response.json().await?;
        Ok(result)
    }

    /// 上传永久图片（获取URL，用于图文消息内的图片）
    pub async fn upload_img(
        &self,
        file_path: &str,
        file_name: &str,
    ) -> Result<UploadImgResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/media/uploadimg?access_token={}",
            access_token
        );

        let file_bytes = tokio::fs::read(file_path).await
            .map_err(|e| format!("读取文件失败: {}", e))?;
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("media", part);

        let response = self.client.post(&url).multipart(form).send().await?;
        let result: UploadImgResponse = response.json().await?;
        Ok(result)
    }

    /// 上传永久图文素材
    pub async fn upload_news(&self, articles: &Vec<NewsArticle>) -> Result<UploadNewsResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/add_news?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "articles": articles
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: UploadNewsResponse = response.json().await?;
        Ok(result)
    }

    /// 修改永久图文素材
    pub async fn update_news(&self, req: &UpdateNewsRequest) -> Result<String> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/update_news?access_token={}",
            access_token
        );

        let response = self.client.post(&url).json(req).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok("修改图文素材成功".to_string())
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("修改图文素材失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }

    /// 获取素材总数
    pub async fn get_material_count(&self) -> Result<MaterialCountResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/get_materialcount?access_token={}",
            access_token
        );

        let response = self.client.get(&url).send().await?;
        let result: MaterialCountResponse = response.json().await?;
        Ok(result)
    }

    /// 获取素材列表（批量获取永久素材）
    pub async fn batch_get_material(
        &self,
        media_type: &str,
        offset: i64,
        count: i64,
    ) -> Result<BatchGetMaterialResponse> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/batchget_material?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "type": media_type,
            "offset": offset,
            "count": count
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: BatchGetMaterialResponse = response.json().await?;
        Ok(result)
    }

    /// 获取永久素材详情
    pub async fn get_material(&self, media_id: &str) -> Result<serde_json::Value> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/get_material?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "media_id": media_id
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }

    /// 删除永久素材
    pub async fn delete_material(&self, media_id: &str) -> Result<String> {
        let access_token = self.get_access_token().await?;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/material/del_material?access_token={}",
            access_token
        );

        let body = serde_json::json!({
            "media_id": media_id
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let result: serde_json::Value = response.json().await?;

        let errcode = result["errcode"].as_i64().unwrap_or(-1);
        if errcode == 0 {
            Ok("删除素材成功".to_string())
        } else {
            let errmsg = result["errmsg"].as_str().unwrap_or("未知错误");
            Err(format!("删除素材失败: {} (错误码: {})", errmsg, errcode).into())
        }
    }
}
