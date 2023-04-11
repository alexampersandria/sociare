use crate::{schema, util};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
  Associations,
  Clone,
  Debug,
  Deserialize,
  Identifiable,
  Insertable,
  PartialEq,
  Queryable,
  Selectable,
  Serialize,
)]
#[diesel(belongs_to(util::User))]
#[diesel(table_name = schema::user_sessions)]
pub struct UserSession {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub accessed_at: i64,
  pub ip_address: String,
  pub user_agent: String,
}

impl UserSession {
  pub fn new(user_id: &str, ip_address: &str, user_agent: &str) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      user_id: user_id.to_string(),
      created_at: util::unix_ms(),
      accessed_at: util::unix_ms(),
      ip_address: ip_address.to_string(),
      user_agent: user_agent.to_string(),
    }
  }
}

#[cfg(test)]
mod ci_unit {
  use crate::util;

  #[test]
  fn new() {
    let user = util::User::new_with_mobilepay("test", "hunter2", "Test User", "e@x.com", "1234");
    let session = util::UserSession::new(&user.id, "ip", "ua");
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.ip_address, "ip");
    assert_eq!(session.user_agent, "ua");
    assert_eq!(session.created_at, session.accessed_at);
  }
}
