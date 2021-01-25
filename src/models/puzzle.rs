use chrono::{DateTime, Utc};
use redis::{ErrorKind, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use std::convert::From;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct PartialPuzzle {
    pub name: String,
    pub released_at: DateTime<Utc>,
    pub words: Vec<String>,
    pub letters: Vec<String>,
    pub middle: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct Puzzle {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub released_at: DateTime<Utc>,
    pub words: Vec<String>,
    pub letters: Vec<String>,
    pub middle: String,
    pub language: String,
}

impl Puzzle {
    pub fn to_key(&self) -> String {
        format!("puzzle:{}", self.id)
    }
}

impl From<PartialPuzzle> for Puzzle {
    fn from(partial: PartialPuzzle) -> Puzzle {
        Puzzle {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),

            name: partial.name,
            released_at: partial.released_at,
            words: partial.words,
            letters: partial.letters,
            middle: partial.middle,
            language: partial.language,
        }
    }
}

impl FromRedisValue for Puzzle {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(v) => {
                let slice: &[u8] = v;
                let result: Puzzle =
                    from_slice(slice).map_err(|_| (ErrorKind::TypeError, "Parsing Error"))?;
                Ok(result)
            }
            _ => Err((ErrorKind::TypeError, "Incorrect redis value for Puzzle"))?,
        }
    }
}

impl ToRedisArgs for Puzzle {
    fn write_redis_args<W: ?Sized>(&self, out: &mut W)
    where
        W: RedisWrite,
    {
        let output: &[u8] = &to_vec(self).expect("Incorrect struct for Puzzle");
        out.write_arg(output);
    }
}
