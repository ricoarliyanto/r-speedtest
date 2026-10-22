use crate::service::prelude::*;
use base64::{engine::general_purpose, Engine};
use tokio::{fs, io::AsyncWriteExt};

pub async fn save_base64_img(base64_data: &str, folder: &str) -> Result<(String, String)> {
    let cleaned_str = remove_prefix(base64_data);
    let server_config = APPCOFIG.server.clone();
    let now = chrono::Local::now();
 
    let file_path_t = format!(
        "{}/{}/{}",
        server_config.static_dir,
        folder,
        now.format("%Y-%m")
    );
    fs::create_dir_all(&file_path_t).await?;
 
    let fid = GID().await;
    let file_name = format!("{}_{}.png", now.format("%d"), fid);
    let file_path = format!("{}/{}", file_path_t, file_name);
 
    let decoded_data = general_purpose::STANDARD.decode(cleaned_str)?;
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(&decoded_data).await?;
 
    let static_dir = strip_data_prefix(&server_config.static_dir);
    let url_path = format!(
        "{}/{}/{}/{}/{}",
        server_config.domainname,
        static_dir,
        folder,
        now.format("%Y-%m"),
        file_name
    );
    let no_domain_path = format!(
        "{}/{}/{}/{}",
        static_dir,
        folder,
        now.format("%Y-%m"),
        file_name
    );
 
    Ok((url_path, no_domain_path))
}

fn remove_prefix(s: &str) -> &str {
    if let Some(send) = s.strip_prefix("data:image/png;base64,") {
        send
    } else {
        s
    }
}

 pub async fn upload_public_file(multipart: Multipart) -> impl IntoResponse {
    let result = save_public_file(multipart).await;
    ApiResponse::from_result(result)
}
 
async fn save_public_file(mut multipart: Multipart) -> Result<String> {
    let field = multipart
        .next_field()
        .await?
        .ok_or("No file field found")?;
 
    let server_config = APPCOFIG.server.clone();
    let content_type = field
        .content_type()
        .map(ToString::to_string)
        .unwrap_or_default();
    let old_url = field
        .file_name()
        .map(ToString::to_string)
        .unwrap_or_default();
    let file_type = get_file_type(&content_type);
    let bytes = field.bytes().await?;
 
    let now = chrono::Local::now();
 
    // 存储路径：static_dir/uploads/YYYY-MM/
    let rel_dir = format!("uploads/{}", now.format("%Y-%m"));
    let file_path_t = format!("{}/{}", server_config.static_dir, rel_dir);
    fs::create_dir_all(&file_path_t).await?;
 
    let fid = GID().await;
    let file_name = format!("{}-{}{}", now.format("%d"), fid, file_type);
    let file_path = format!("{}/{}", file_path_t, file_name);
 
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(&bytes).await?;
 
    // 删除旧文件（如果有）
    if !old_url.is_empty() {
        delete_public_file(&old_url).await;
    }
 
    // 返回可直接访问的 URL
    let static_dir = strip_data_prefix(&server_config.static_dir);
    let url_path = format!(
        "{}/{}/{}/{}",
        server_config.domainname,
        static_dir,
        rel_dir,
        file_name
    );
 
    Ok(url_path)
}

fn strip_data_prefix(path: &str) -> &str {
    path.strip_prefix("data/").unwrap_or(path)
}
 
pub async fn delete_public_file(url: &str) {
    let server_config = APPCOFIG.server.clone();
    // 把 URL 中的域名替换为本地 static_dir，还原出真实路径
    let path = url.replace(&server_config.domainname, &server_config.static_dir);
    if fs::remove_file(&path).await.is_err() {
        tracing::error!("Public file deletion failed: {}", path);
    }
}

pub async fn upload_private_file(
    claims: Claims, // 上传本身也需要鉴权
    multipart: Multipart,
) -> impl IntoResponse {
    let result = save_private_file(claims, multipart).await;
    ApiResponse::from_result(result)
}
 
async fn save_private_file(_claims: Claims, mut multipart: Multipart) -> Result<String> {
    let field = multipart
        .next_field()
        .await?
        .ok_or("No file field found")?;
 
    let server_config = APPCOFIG.server.clone();
    let content_type = field
        .content_type()
        .map(ToString::to_string)
        .unwrap_or_default();
    let file_type = get_file_type(&content_type);
    let bytes = field.bytes().await?;
 
    let now = chrono::Local::now();
 
    // 存储路径：upload_dir/YYYY-MM/
    let file_path_t = format!(
        "{}/{}",
        server_config.upload_dir,
        now.format("%Y-%m")
    );
    fs::create_dir_all(&file_path_t).await?;
 
    let fid = GID().await;
    let file_name = format!("{}-{}{}", now.format("%d"), fid, file_type);
    let file_path = format!("{}/{}", file_path_t, file_name);
 
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(&bytes).await?;
 
    // 返回相对路径（文件 ID），不暴露真实磁盘路径，也不生成可直接访问的 URL
    // 前端拿到这个 file_key，之后通过 /files/download/:file_key 下载
    let file_key = format!("{}/{}", now.format("%Y-%m"), file_name);
    Ok(file_key)
}

///私有文件下载 待实现
pub async fn download_private_file(
    Path(file_key): Path<String>,
    claims: Claims,  
) -> impl IntoResponse {

}

fn get_file_type(content_type: &str) -> String {
    match content_type {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        "image/gif" => ".gif".to_string(),
        _ => "".to_string(),
    }
}
