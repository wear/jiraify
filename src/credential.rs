use base64::{encode};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseAuthCredential {
  pub user: String,
  pub access_token: String
}

impl ::std::default::Default for BaseAuthCredential {
    fn default() -> Self { Self { user: "".to_string(), access_token: "".to_string() } }
}


impl BaseAuthCredential {
  pub fn from(user: String, access_token: String) -> Self {
    Self {
      user,
      access_token,
    }
  }

  pub fn secret(&self) -> String {
    let secret = &format!("{user}:{access_token}", user=self.user, access_token=self.access_token);
    format!("Basic {}", encode(secret))
  }
}

#[test]
fn generate_secret() {
  let credit = BaseAuthCredential::from("fred".to_string(), "fred".to_string());
  assert_eq!(credit.secret(), "Basic ZnJlZDpmcmVk");
}

