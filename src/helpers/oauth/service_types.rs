use crate::helpers::config;

#[derive(Debug, Copy, Clone)]
pub enum ServiceType {
    Yandex,
    VK,
}

impl ServiceType {
    pub fn get_information(&self) -> ServiceInformation {
        let service_type_string = self.to_string().to_uppercase();
        let client_id = config::get(service_type_string.to_owned() + "_CLIENT_ID");
        let client_secret =
            config::get(service_type_string.to_owned() + "_CLIENT_SECRET");

        let url = config::get(service_type_string.to_owned() + "_URL");
        let token_url = config::get(service_type_string.to_owned() + "_TOKEN_URL");
        let api_url = config::get(service_type_string.to_owned() + "_API_URL");
        let redirect_uri = config::get(service_type_string.to_owned() + "_REDIRECT_URI");

        ServiceInformation {
            client_id,
            client_secret,
            url,
            token_url,
            api_url,
            redirect_uri,
        }
    }
}

impl std::string::ToString for ServiceType {
    fn to_string(&self) -> String {
        match self {
            Self::Yandex => "yandex".to_string(),
            Self::VK => "vk".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ServiceInformation {
    pub client_id: String,
    pub client_secret: String,
    pub url: String,
    pub token_url: String,
    pub api_url: String,
    pub redirect_uri: String,
}

