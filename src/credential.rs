use base64::{encode};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JiraCredential {
  pub email: String,
  pub api_token: String
}


impl ::std::default::Default for JiraCredential {
    fn default() -> Self { Self { email: "".to_string(), api_token: "".to_string() } }
}

impl JiraCredential {
  pub fn from(email: String, api_token: String) -> JiraCredential {
    JiraCredential {
      email,
      api_token
    }
  }

  pub fn secret(&self) -> String {
    let secret = &format!("{email}:{api_key}", email=self.email, api_key=self.api_token);
    format!("Basic {}", encode(secret))
  }
}

#[test]
fn generate_secret() {
  let credit = JiraCredential::from("fred".to_string(), "fred".to_string());
  assert_eq!(credit.secret(), "Basic ZnJlZDpmcmVk");
}

