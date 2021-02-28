use chrono::NaiveDateTime;
use sqlx::error::Error;
use sqlx::PgPool;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    uid: i64,
    username: String,
    #[serde(skip_serializing)]
    password: String,
    email:String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    sex: bool
}

type Users = Vec<User>;

impl User {
    pub async fn create(&self,db: &PgPool) -> Result<i64,Error>{
        sqlx::query_unchecked!(r#"INSERT INTO accounts (uid, username, password,email,created_at,updated_at,sex) VALUES ($1, $2, $3,$4, $5,$6, $7) RETURNING uid"#,&self.uid,&self.username,&self.password,&self.email,&self.created_at,&self.updated_at,&self.sex).execute(db).await?
    }
}