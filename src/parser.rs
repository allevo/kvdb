/// There are 3 types of statement in KVDB, GET/SET/DEL.
#[derive(PartialEq)]
pub enum StatementType {
    /// Relates to the set() method of the Storage Engine.
    Set,
    /// Relates to the get() method of the Storage Engine.
    Get,
    /// Relates to the del() method of the Storage Engine.
    Del,
    /// No such operation exists.
    Unk,
    /// The parser has failed to understand what the user wants
    /// to do. Parsing has failed due to wrong command syntax.
    Fail,
}

impl StatementType {
    /// Convert written operation keywords into enum symbols.
    fn check(word: &str) -> Self {
        match word.to_lowercase().as_ref() {
            "set" | "put" | "insert" | "in" | "i" => Self::Set,
            "get" | "select" | "output" | "out" | "o" => Self::Get,
            "del" | "delete" | "rem" | "remove" | "rm" | "d" => Self::Del,
            _ => Self::Unk,
        }
    }

    /// Get string form of command words from the StatementType object.
    fn get_word(&self) -> String {
        match self {
            Self::Set => "SET".to_string(),
            Self::Get => "GET".to_string(),
            Self::Del => "DEL".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

/// Describes the structure of a REPL statement.
#[derive(PartialEq)]
pub struct Statement {
    /// Depicts the type of Operation the statement conveys.
    pub stype: StatementType,
    /// The key variable, only used in get/set/del statements.
    pub key: Option<String>,
    /// The value variable, only used in set statements.
    pub value: Option<String>,
}

impl Statement {
    /// Creates a REPL statement from user input command.
    pub fn prep(cmd: &String) -> Self {
        // Divide user input into words.
        let cmd_words: Vec<&str> = cmd.split(|c| c == ' ' || c == '\t').collect();
        // Find statement type.
        let stype = StatementType::check(cmd_words[0]);
        // Collect rest of the words, if exists, into a single string.
        let cmd_val = match cmd_words.len() > 1 {
            true => cmd_words[2..].to_vec().join(" ").trim().to_string(),
            false => "".to_string(),
        };

        // The first word after the operation keyword is supposed to be
        // the statement key, else the statement has failed to parse.
        let key = match stype {
            StatementType::Get | StatementType::Set | StatementType::Del => {
                if cmd_words.len() < 2 {
                    // Incase the user forgets to input required options
                    // for an operation, fail by setting None.
                    eprintln!(
                        "Error: `{}` operation ignored, KEY not provided.",
                        stype.get_word()
                    );
                    None
                } else {
                    Some(cmd_words[1].to_string())
                }
            }
            _ => None,
        };

        // The string after the operation keyword and the statement key
        // is the statement value. Parsing should fail if no such value
        // for the `set` operation. Currently, the code sets value to an
        // empty string value.
        let value = match stype {
            StatementType::Set => {
                if cmd_words.len() < 3 {
                    // Incase the user forgets to input required options
                    // for an operation, fail by setting None.
                    eprintln!(
                        "Error: `{}` operation ignored, VALUE not provided.",
                        stype.get_word()
                    );
                    None
                } else {
                    Some(cmd_val)
                }
            }
            StatementType::Get | StatementType::Del => {
                if cmd_words.len() > 2 {
                    // Incase the user unnecessarily inputs a value for either
                    // GET or DEL operations, warn them and don't use the value.
                    eprintln!("Warning: Too many inputs, `{}` was ignored.", cmd_val);
                }
                None
            }
            _ => None,
        };

        // Quick Fix to #1. If for most operations key is set to None and for set operation only,
        // if value is set to None, set stype to Fail to fail parsing. All Unk operations are passed as is.
        if (stype == StatementType::Set && value.is_none())
            || (stype != StatementType::Unk && key.is_none())
        {
            // Fail state, when user forgets to pass necessary inputs.
            Self {
                stype: StatementType::Fail,
                key: None,
                value: None,
            }
        } else {
            Self { stype, key, value }
        }
    }
}
