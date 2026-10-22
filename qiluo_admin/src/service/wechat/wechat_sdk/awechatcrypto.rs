use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, KeyInit};
use aes::Aes256;
use base64::{alphabet, engine};
use base64::{engine::general_purpose, Engine as _};
use cbc::cipher::{BlockEncryptMut, KeyIvInit};
use cbc::Encryptor;
use sha1::{Digest, Sha1};

type Aes256CbcEnc = Encryptor<Aes256>;

#[derive(Debug, Clone)]
pub struct WechatCrypto {
    pub token: String,
    pub encoding_aes_key: String,
    pub app_id: String,
    aes_key: Vec<u8>,
}

impl WechatCrypto {
    pub fn new(token: String, encoding_aes_key: String, app_id: String) -> Result<Self, String> {
        let aes_key =
            Self::decode_like(&encoding_aes_key).map_err(|e| format!("解码AES密钥失败: {}", e))?;

        if aes_key.len() != 32 {
            return Err(format!(
                "AES密钥长度错误，期望32字节，实际{}字节",
                aes_key.len()
            ));
        }

        Ok(Self {
            token,
            encoding_aes_key,
            app_id,
            aes_key,
        })
    }

    fn decode_like(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let alphabet = alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        )?;

        let config = engine::GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(true)
            .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);

        let lenient_engine = engine::GeneralPurpose::new(&alphabet, config);

        Ok(lenient_engine.decode(input)?)
    }

    fn process_encoding_aes_key(key: &str) -> Result<String, String> {
        let mut processed_key = key.to_string();

        processed_key = processed_key
            .replace('-', "+")
            .replace('_', "/");

        match processed_key.len() {
            43 => {
                processed_key.push('=');
            }
            44 => {}
            42 => {
                processed_key.push_str("==");
            }
            len => {
                return Err(format!("无效的EncodingAESKey长度: {}, 期望42-44位", len));
            }
        }

        if !Self::is_valid_base64(&processed_key) {
            return Err(format!("EncodingAESKey包含无效字符: {}", key));
        }

        Ok(processed_key)
    }

    fn is_valid_base64(s: &str) -> bool {
        const BASE64_CHARS: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
        s.bytes().all(|b| BASE64_CHARS.contains(&b))
    }

    pub fn verify_url_signature(
        &self,
        signature: &str,
        timestamp: &str,
        nonce: &str,
        echostr: &str,
    ) -> bool {
        let mut params = vec![&self.token, timestamp, nonce, echostr];
        params.sort();
        let content = params.join("");

        let mut hasher = Sha1::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        let computed_signature = hex::encode(result);

        computed_signature == signature
    }

    pub fn verify_message_signature(
        &self,
        signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypt_msg: &str,
    ) -> bool {
        let mut params = vec![&self.token, timestamp, nonce, encrypt_msg];
        params.sort();
        let content = params.join("");

        let mut hasher = Sha1::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        let computed_signature = hex::encode(result);

        computed_signature == signature
    }

    pub fn decrypt_message(&self, encrypt_msg: &str) -> Result<String, String> {
        let encrypted_data = general_purpose::STANDARD
            .decode(encrypt_msg)
            .map_err(|e| format!("Base64解码失败: {}", e))?;

        let mut msg = vec![0u8; encrypted_data.len() + 32 - encrypted_data.len() % 32];
        msg[..encrypted_data.len()].copy_from_slice(&encrypted_data);

        let key = GenericArray::from_slice(&self.aes_key);
        let iv = GenericArray::from_slice(&self.aes_key[0..16]);

        let decrypted_data = self.aes_decrypt_no_padding(&encrypted_data, iv, key)?;
        let cleaned_data = self.remove_padding(&decrypted_data)?;
        self.parse_decrypted_data(&cleaned_data)
    }

    fn aes_decrypt_no_padding(
        &self,
        input: &[u8],
        iv: &GenericArray<u8, aes::cipher::typenum::U16>,
        key: &GenericArray<u8, aes::cipher::typenum::U32>,
    ) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut previous_block = iv.clone();

        for chunk in input.chunks(16) {
            if chunk.len() != 16 {
                continue;
            }

            let mut block = GenericArray::clone_from_slice(chunk);

            let cipher = Aes256::new(key);
            cipher.decrypt_block(&mut block);

            for (i, byte) in block.iter_mut().enumerate() {
                *byte ^= previous_block[i];
            }

            result.extend_from_slice(&block);
            previous_block = GenericArray::clone_from_slice(chunk);
        }

        Ok(result)
    }

    fn remove_padding(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        if data.is_empty() {
            return Err("解密数据为空".to_string());
        }

        let mut end_pos = data.len();

        if let Some(&last_byte) = data.last() {
            while end_pos > 0 && data[end_pos - 1] == last_byte {
                end_pos -= 1;
            }

            let padding_length = data.len() - end_pos;
            if padding_length > 0 && padding_length <= 32 && last_byte as usize == padding_length {
                return Ok(data[..end_pos].to_vec());
            }

            while end_pos > 0 && data[end_pos - 1] < 32 {
                end_pos -= 1;
            }
        }

        Ok(data[..end_pos].to_vec())
    }

    fn parse_decrypted_data(&self, decrypted_bytes: &[u8]) -> Result<String, String> {
        if decrypted_bytes.len() < 20 {
            return Err("解密数据长度不足".to_string());
        }

        let _random = &decrypted_bytes[0..16];

        let msg_len_bytes = &decrypted_bytes[16..20];
        let msg_len = u32::from_be_bytes([
            msg_len_bytes[0],
            msg_len_bytes[1],
            msg_len_bytes[2],
            msg_len_bytes[3],
        ]) as usize;

        if decrypted_bytes.len() < 20 + msg_len {
            return Err("解密数据长度与消息长度不匹配".to_string());
        }

        let msg_bytes = &decrypted_bytes[20..20 + msg_len];
        let xml_content = String::from_utf8(msg_bytes.to_vec())
            .map_err(|e| format!("消息内容UTF-8解码失败: {}", e))?;

        let app_id_bytes = &decrypted_bytes[20 + msg_len..];
        let received_app_id = String::from_utf8(app_id_bytes.to_vec())
            .map_err(|e| format!("AppId UTF-8解码失败: {}", e))?;

        if received_app_id.trim() != self.app_id {
            return Err(format!(
                "AppId验证失败，期望: {}, 实际: {}",
                self.app_id, received_app_id
            ));
        }

        Ok(xml_content)
    }

    pub fn encrypt_message(
        &self,
        reply_msg: &str,
        timestamp: Option<u64>,
    ) -> Result<String, String> {
        let random_bytes = self.generate_random_bytes(16);

        let msg_bytes = reply_msg.as_bytes();
        let msg_len = msg_bytes.len() as u32;
        let app_id_bytes = self.app_id.as_bytes();

        let mut plaintext = Vec::new();
        plaintext.extend_from_slice(&random_bytes);
        plaintext.extend_from_slice(&msg_len.to_be_bytes());
        plaintext.extend_from_slice(msg_bytes);
        plaintext.extend_from_slice(app_id_bytes);

        let key = GenericArray::from_slice(&self.aes_key);
        let iv = GenericArray::from_slice(&self.aes_key[0..16]);
        let encryptor = Aes256CbcEnc::new(key, iv);

        let original_len = plaintext.len();

        let block_size = 16;
        let padding_len = block_size - (original_len % block_size);
        let total_len = original_len + padding_len;

        plaintext.resize(total_len, 0);

        let ciphertext = encryptor
            .encrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut plaintext, original_len)
            .map_err(|e| format!("加密失败: {:?}", e))?;

        let encrypt_msg = general_purpose::STANDARD.encode(&ciphertext);

        let timestamp_str = timestamp
            .unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            })
            .to_string();

        let nonce = self.generate_nonce();
        let signature = self.generate_signature(&timestamp_str, &nonce, &encrypt_msg);

        let encrypted_xml = format!(
            r#"<xml>
<Encrypt><![CDATA[{}]]></Encrypt>
<MsgSignature><![CDATA[{}]]></MsgSignature>
<TimeStamp>{}</TimeStamp>
<Nonce><![CDATA[{}]]></Nonce>
</xml>"#,
            encrypt_msg, signature, timestamp_str, nonce
        );

        Ok(encrypted_xml)
    }

    fn generate_random_bytes(&self, len: usize) -> Vec<u8> {
        use rand::RngExt;
        let mut rng = rand::rng();
        (0..len).map(|_| rng.random()).collect()
    }

    fn generate_nonce(&self) -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        use rand::RngExt;
        let mut rng = rand::rng();
        (0..16)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    fn generate_signature(&self, timestamp: &str, nonce: &str, encrypt_msg: &str) -> String {
        let mut params = vec![&self.token, timestamp, nonce, encrypt_msg];
        params.sort();
        let content = params.join("");

        let mut hasher = Sha1::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}
