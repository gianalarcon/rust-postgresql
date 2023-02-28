// Define the model in a struct
#[derive(Serialize, Deserialize, Debug)]

pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}
