use base64::{encode};

pub struct JiraCredential<'a> {
  email: &'a str,
  api_token: &'a str
}

impl<'a> JiraCredential<'a> {
  pub fn from(email: &'a str, api_token: &'a str) -> JiraCredential<'a> {
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
  let credit = JiraCredential::from("fred", "fred");
  assert_eq!(credit.secret(), "Basic ZnJlZDpmcmVk");
}

