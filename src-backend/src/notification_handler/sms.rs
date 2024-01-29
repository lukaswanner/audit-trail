use reqwest::Client;

#[derive(Clone)]
pub struct SmsSettings {
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub twilio_phone_number: String,
    pub twilio_api_url: String,
}

#[derive(Clone)]
pub struct SmsParams {
    pub to: String,
    pub body: String,
}

#[derive(Clone)]
pub struct Sms {
    pub settings: SmsSettings,
    pub sms_params: SmsParams,
}

impl SmsParams {
    pub fn default() -> SmsParams {
        SmsParams {
            to: String::from(""),
            body: String::from(""),
        }
    }
    pub fn new(to: String, body: String) -> SmsParams {
        SmsParams { to, body }
    }
}

impl SmsSettings {
    fn new_from_env() -> SmsSettings {
        let account_sid = std::env::var("ACCOUNT_SID").expect("ACCOUNT_SID must be set");
        let auth_token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set");
        let phone_number = std::env::var("PHONE_NUMBER").expect("PHONE_NUMBER must be set");
        let api_url = std::env::var("API_URL").expect("API_URL must be set");

        SmsSettings {
            twilio_account_sid: account_sid,
            twilio_auth_token: auth_token,
            twilio_phone_number: phone_number,
            twilio_api_url: api_url,
        }
    }
}

impl Sms {
    pub fn new() -> Sms {
        Sms {
            settings: SmsSettings::new_from_env(),
            sms_params: SmsParams::default(),
        }
    }

    pub fn set_params(&mut self, to: String, body: String) -> &mut Self {
        self.sms_params = SmsParams::new(to, body);
        self
    }

    pub async fn send(&self) -> () {
        let client = Client::new();

        let request_params = [
            ("To", &self.sms_params.to),
            ("From", &self.settings.twilio_phone_number),
            ("Body", &self.sms_params.body),
        ];

        let response = client
            .post(&self.settings.twilio_api_url)
            .basic_auth(
                &self.settings.twilio_account_sid,
                Some(&self.settings.twilio_auth_token),
            )
            .form(&request_params)
            .send()
            .await
            .unwrap();

        println!("Response: {:?}", response.status().to_string());
    }
}
