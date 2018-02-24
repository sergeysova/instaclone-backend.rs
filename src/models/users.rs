use super::super::schema::users;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
  pub id: i32,
  pub some_field: Option<String>
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[table_name="users"]
pub struct NewUser<'a> {
  pub some_field: &'a str
}
