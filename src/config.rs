pub struct Config {
    pub author: Author,
}

pub struct Author {
    pub name: String,
    pub email: String,
}

impl Default for Author {
    fn default() -> Self {
        Author {
            name: "Default".to_string(),
            email: "user@local".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        // open file from the .rev/config file
        let config_file = std::fs::read_to_string(".rev/config").unwrap_or_default();

        if config_file.is_empty() {
            return Config {
                author: Author::default(),
            };
        }

        let author_name = config_file
            .lines()
            .find(|line| line.starts_with("author.name = "))
            .and_then(|line| line.split(" = ").nth(1))
            .unwrap_or("Default")
            .to_string();

        let author_email = config_file
            .lines()
            .find(|line| line.starts_with("author.email = "))
            .and_then(|line| line.split(" = ").nth(1))
            .unwrap_or("user@local")
            .to_string();

        Self {
            author: Author {
                name: author_name,
                email: author_email,
            },
        }
    }
}
