#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum QueryType {
    Unknown(u16),
    A,
    NS,
    CNAME,
    MX,
    AAAA,
}

impl QueryType {
    pub fn from_u16(value: u16) -> QueryType {
        match value {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            28 => QueryType::AAAA,
            _ => QueryType::Unknown(value),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::CNAME => 5,
            QueryType::MX => 15,
            QueryType::AAAA => 28,
            QueryType::Unknown(value) => value,
        }
    }
}

impl std::str::FromStr for QueryType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(QueryType::A),
            "NS" => Ok(QueryType::NS),
            "CNAME" => Ok(QueryType::CNAME),
            "MX" => Ok(QueryType::MX),
            "AAAA" => Ok(QueryType::AAAA),
            _ => Err(anyhow::anyhow!("Unknown query type")),
        }
    }
}
