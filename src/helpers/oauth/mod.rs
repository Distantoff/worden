use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenUrl, AuthType, TokenResponse,
    reqwest::async_http_client
};
use serde::{Deserialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::fmt::Debug;
use url_builder::URLBuilder;
use myclient::MyClient;
use service_types::*;

pub mod service_types;

impl OAuth {
    pub fn new(service_type: ServiceType) -> Self {
        let client = Self::get_client(service_type);
        OAuth { service_type, client }
    }

    fn get_client(service_type: ServiceType) -> MyClient {
        let service_info = service_type.get_information();
        let client_id = ClientId::new(service_info.client_id);
        let client_secret = ClientSecret::new(service_info.client_secret);
        let auth_url = AuthUrl::new(service_info.url).unwrap();
        let token_url = TokenUrl::new(service_info.token_url).unwrap();
        let redirect_uri = RedirectUrl::new(service_info.redirect_uri).unwrap();

        let client = MyClient::new(
            client_id, Some(client_secret),
            auth_url, Some(token_url)
        )
        .set_auth_type(AuthType::RequestBody)
        .set_redirect_uri(redirect_uri);

        client
    }

    pub async fn fetch_email(&self, parameters: HashMap<String, String>)
        -> Option<String> {
        if let Some(code) = parameters.get("code") {
            let email = match self.service_type {
                ServiceType::Yandex =>
                    self.fetch_information::<YandexResponse>(code)
                        .await.unwrap().email,

                ServiceType::VK =>
                    self.fetch_information::<VkResponse>(code)
                        .await.unwrap().email,
            };

            return Some(email);
        }

        None
    }

    async fn fetch_information<SR: SocialResponse + DeserializeOwned + Debug>
        (&self, code: &String) -> Option<SR> {

        let token_result =
            self.client.clone()
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .request_async(async_http_client)
            .await;

        if let Some(email) = &token_result.as_ref().unwrap().extra_fields.email {
            return Some(SR::new(email.to_string()));
        }

        let access_token =
            token_result.as_ref().unwrap().access_token().secret().as_str();
        let url = SR::get_url(access_token.to_string());
        let response = reqwest::get(&url).await
            .unwrap().json::<SR>().await.unwrap();

        Some(response)
    }

    pub fn get_authorize_url(&self) -> String {
        let (authorize_url, _) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("login:email".to_string()))
            .url();

        authorize_url.to_string()
    }
}

pub struct OAuth {
    service_type: ServiceType,
    client: MyClient
}

pub trait SocialResponse {
    fn new(email: String) -> Self;
    fn get_url(access_token: String) -> String;
}

impl SocialResponse for VkResponse {
    fn new(email: String) -> Self {
        Self { email: email.to_string() }
    }

    fn get_url(access_token: String) -> String {
        let mut ub = URLBuilder::new();
        ub.set_protocol("https")
            .set_host("api.vk.com")
            .add_route("method")
            .add_route("users.get")
            .add_param("v", "5.81")
            .add_param("fields", "uid,screen_name,e-mail")
            .add_param("access_token", access_token.as_str());

        ub.build()
    }
}

impl SocialResponse for YandexResponse {
    fn new(email: String) -> Self {
        Self { email: email.to_string() }
    }

    fn get_url(access_token: String) -> String {
        let mut ub = URLBuilder::new();
        ub.set_protocol("https")
            .set_host("login.yandex.ru")
            .add_route("info")
            .add_param("format", "json")
            .add_param("oauth_token", &access_token);

        ub.build()
    }
}

#[derive(Deserialize, Debug)]
struct YandexResponse {
    #[serde(rename = "default_email")]
    email: String,
}

#[derive(Deserialize, Debug)]
struct VkResponse {
    email: String,
}

mod myclient {
    // Создаем свою структуру с опциональным token_type из-за того, что
    // VK не соответствет протоколу oauth и не возвращает обязательный
    // token_type, который обычно является bearer, из-за чего происходит
    // ошибка парсинга

    use oauth2::{
        basic::*, Scope, TokenType, ExtraTokenFields, TokenResponse,
        AccessToken, RefreshToken, Client, StandardRevocableToken, 
        StandardTokenIntrospectionResponse
    };

    use core::time::Duration;
    use serde::{Deserialize, Serialize};

    pub type MyClient = Client<
        BasicErrorResponse,
        MyTokenResponse<VkExtraTokenFields, BasicTokenType>,
        BasicTokenType,
        StandardTokenIntrospectionResponse<
            VkExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
    >;

    #[derive(Deserialize, Debug, Serialize, Clone)]
    pub struct VkExtraTokenFields {
        pub email: Option<String>,
    }

    impl ExtraTokenFields for VkExtraTokenFields { }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MyTokenResponse<EF, TT>
    where
        EF: ExtraTokenFields,
        TT: TokenType,
    {
        access_token: AccessToken,
        #[serde(bound = "TT: TokenType")]
        token_type: Option<TT>,
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_in: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        refresh_token: Option<RefreshToken>,
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        scopes: Option<Vec<Scope>>,
        #[serde(bound = "EF: ExtraTokenFields")]
        #[serde(flatten)]
        pub extra_fields: EF,
    }

    impl<EF, TT> TokenResponse<TT> for MyTokenResponse<EF, TT>
    where
        EF: ExtraTokenFields,
        TT: TokenType,
    {
        fn access_token(&self) -> &AccessToken {
            &self.access_token
        }
        fn token_type(&self) -> &TT {
            &self.token_type.as_ref().unwrap()
        }
        fn expires_in(&self) -> Option<Duration> {
            self.expires_in.map(Duration::from_secs)
        }
        fn refresh_token(&self) -> Option<&RefreshToken> {
            self.refresh_token.as_ref()
        }
        fn scopes(&self) -> Option<&Vec<Scope>> {
            self.scopes.as_ref()
        }
    }
}
