#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum QueryType {
    Unknown(u16),
    A,
}

impl QueryType {
    pub fn from_u16(value: u16) -> QueryType {
        match value {
            1 => QueryType::A,
            _ => QueryType::Unknown(value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            QueryType::A => 1,
            QueryType::Unknown(value) => value,
        }
    }
}

impl std::str::FromStr for QueryType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(QueryType::A),
            _ => Err(anyhow::anyhow!("Unknown query type")),
        }
    }
}
