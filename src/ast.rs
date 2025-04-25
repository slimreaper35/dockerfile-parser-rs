// https://docs.docker.com/reference/dockerfile/#overview

use std::collections::BTreeMap;
use std::fmt;

use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
/// This enum represents available protocols for the EXPOSE instruction in a Dockerfile.
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug)]
/// This enum represents available instructions in a Dockerfile.
pub enum Instruction {
    ADD {
        checksum: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    ARG(BTreeMap<String, Option<String>>),
    CMD(Vec<String>),
    COPY {
        from: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    ENTRYPOINT(Vec<String>),
    ENV(BTreeMap<String, String>),
    EXPOSE {
        port: String,
        protocol: Option<Protocol>,
    },
    FROM {
        platform: Option<String>,
        image: String,
        alias: Option<String>,
    },
    LABEL(BTreeMap<String, String>),
    RUN {
        mount: Option<String>,
        network: Option<String>,
        security: Option<String>,
        command: Vec<String>,
    },
    SHELL(Vec<String>),
    STOPSIGNAL {
        signal: String,
    },
    USER {
        user: String,
        group: Option<String>,
    },
    VOLUME {
        mounts: Vec<String>,
    },
    WORKDIR {
        path: String,
    },
    //-------------//
    //    extra    //
    //-------------//
    COMMENT(String),
    EMPTY,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::ADD {
                checksum,
                chown,
                chmod,
                link,
                sources,
                destination,
            } => {
                let options = vec![
                    helpers::format_instruction_option("checksum", checksum),
                    helpers::format_instruction_option("chown", chown),
                    helpers::format_instruction_option("chmod", chmod),
                    helpers::format_instruction_option("link", link),
                ];
                let options_string = helpers::format_options_string(options);
                let prefix = if options_string.is_empty() {
                    String::new()
                } else {
                    format!("{} ", options_string)
                };
                write!(f, "ADD {}{} {}", prefix, sources.join(" "), destination)
            }
            Instruction::ARG(args) => {
                let arg_string = args
                    .iter()
                    .map(|(key, value)| {
                        if let Some(default) = value {
                            format!("{}={}", key, default)
                        } else {
                            key.to_owned()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "ARG {}", arg_string)
            }
            Instruction::CMD(cmd) => write!(f, "CMD {:?}", cmd),
            Instruction::COPY {
                from,
                chown,
                chmod,
                link,
                sources,
                destination,
            } => {
                let options = vec![
                    helpers::format_instruction_option("from", from),
                    helpers::format_instruction_option("chown", chown),
                    helpers::format_instruction_option("chmod", chmod),
                    helpers::format_instruction_option("link", link),
                ];
                let options_string = helpers::format_options_string(options);
                let prefix = if options_string.is_empty() {
                    String::new()
                } else {
                    format!("{} ", options_string)
                };
                write!(f, "COPY {}{} {}", prefix, sources.join(" "), destination)
            }
            Instruction::ENTRYPOINT(entrypoint) => write!(f, "ENTRYPOINT {:?}", entrypoint),
            Instruction::ENV(env) => {
                write!(f, "ENV {}", helpers::format_btree_map(env))
            }
            Instruction::EXPOSE { port, protocol } => {
                if let Some(protocol) = protocol {
                    write!(f, "EXPOSE {}/{}", port, protocol)
                } else {
                    write!(f, "EXPOSE {}", port)
                }
            }
            Instruction::FROM {
                platform,
                image,
                alias,
            } => {
                let options = vec![helpers::format_instruction_option("platform", platform)];
                let options_string = helpers::format_options_string(options);
                let prefix = if options_string.is_empty() {
                    String::new()
                } else {
                    format!("{} ", options_string)
                };
                let mut line = format!("FROM {}{}", prefix, image);

                if let Some(alias) = alias {
                    line.push_str(&format!(" AS {}", alias));
                }
                write!(f, "{}", line)
            }
            Instruction::LABEL(labels) => {
                write!(f, "LABEL {}", helpers::format_btree_map(labels))
            }
            Instruction::RUN {
                mount,
                network,
                security,
                command,
            } => {
                let options = vec![
                    helpers::format_instruction_option("mount", mount),
                    helpers::format_instruction_option("network", network),
                    helpers::format_instruction_option("security", security),
                ];
                let options_string = helpers::format_options_string(options);
                let prefix = if options_string.is_empty() {
                    String::new()
                } else {
                    format!("{} ", options_string)
                };
                write!(f, "RUN {}{:?}", prefix, command)
            }
            Instruction::SHELL(shell) => write!(f, "SHELL {:?}", shell),
            Instruction::STOPSIGNAL { signal } => write!(f, "STOPSIGNAL {}", signal),
            Instruction::USER { user, group } => {
                if let Some(group) = group {
                    write!(f, "USER {}:{}", user, group)
                } else {
                    write!(f, "USER {}", user)
                }
            }
            Instruction::VOLUME { mounts } => write!(f, "VOLUME {:?}", mounts),
            Instruction::WORKDIR { path } => write!(f, "WORKDIR {}", path),
            //-------------//
            //    extra    //
            //-------------//
            Instruction::COMMENT(comment) => write!(f, "{}", comment),
            Instruction::EMPTY => write!(f, ""),
        }
    }
}

mod helpers {
    use super::*;

    pub fn format_instruction_option(key: &str, value: &Option<String>) -> String {
        value
            .as_ref()
            .map(|v| format!("--{}={}", key, v))
            .unwrap_or_default()
    }

    pub fn format_options_string(options: Vec<String>) -> String {
        options
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn format_btree_map(pairs: &BTreeMap<String, String>) -> String {
        pairs
            .iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_instruction_add() {
        let instruction = Instruction::ADD {
            checksum: None,
            chown: None,
            chmod: None,
            link: None,
            sources: vec![String::from("source1"), String::from("source2")],
            destination: String::from("/destination"),
        };

        let expected = "ADD source1 source2 /destination";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_arg() {
        let instruction = Instruction::ARG(BTreeMap::from([
            (String::from("ARG2"), None),
            (String::from("ARG1"), Some(String::from("value1"))),
        ]));

        // must be sorted
        let expected = "ARG ARG1=value1 ARG2";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_cmd() {
        let instruction =
            Instruction::CMD(vec![String::from("echo"), String::from("Hello, World!")]);

        let expected = "CMD [\"echo\", \"Hello, World!\"]";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_copy() {
        let instruction = Instruction::COPY {
            from: Some(String::from("builder")),
            chown: None,
            chmod: None,
            link: None,
            sources: vec![String::from("source1"), String::from("source2")],
            destination: String::from("/destination"),
        };

        let expected = "COPY --from=builder source1 source2 /destination";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_entrypoint() {
        let instruction = Instruction::ENTRYPOINT(vec![String::from("entrypoint.sh")]);

        let expected = "ENTRYPOINT [\"entrypoint.sh\"]";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_env() {
        let instruction = Instruction::ENV(BTreeMap::from([
            (String::from("ENV2"), String::from("value2")),
            (String::from("ENV1"), String::from("value1")),
        ]));

        // must be sorted
        let expected = "ENV ENV1=\"value1\" ENV2=\"value2\"";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_expose() {
        let instruction = Instruction::EXPOSE {
            port: String::from("8080"),
            protocol: Some(Protocol::Udp),
        };

        let expected = "EXPOSE 8080/udp";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_from() {
        let instruction = Instruction::FROM {
            platform: Some(String::from("linux/amd64")),
            image: String::from("docker.io/library/fedora:latest"),
            alias: Some(String::from("builder")),
        };

        let expected = "FROM --platform=linux/amd64 docker.io/library/fedora:latest AS builder";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_label() {
        let instruction = Instruction::LABEL(BTreeMap::from([
            (String::from("version"), String::from("1.0")),
            (String::from("maintainer"), String::from("John Doe")),
        ]));

        // must be sorted
        let expected = "LABEL maintainer=\"John Doe\" version=\"1.0\"";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_run() {
        let instruction = Instruction::RUN {
            mount: None,
            network: None,
            security: None,
            command: vec![String::from("echo"), String::from("Hello, World!")],
        };

        let expected = "RUN [\"echo\", \"Hello, World!\"]";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_shell() {
        let instruction = Instruction::SHELL(vec![String::from("/bin/sh"), String::from("-c")]);

        let expected = "SHELL [\"/bin/sh\", \"-c\"]";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_user() {
        let instruction = Instruction::USER {
            user: String::from("root"),
            group: Some(String::from("root")),
        };

        let expected = "USER root:root";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_volume() {
        let instruction = Instruction::VOLUME {
            mounts: vec![String::from("/data"), String::from("/var/log")],
        };

        let expected = "VOLUME [\"/data\", \"/var/log\"]";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_workdir() {
        let instruction = Instruction::WORKDIR {
            path: String::from("/app"),
        };

        let expected = "WORKDIR /app";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_comment() {
        let instruction = Instruction::COMMENT(String::from("# This is a comment"));

        let expected = "# This is a comment";
        assert_eq!(format!("{}", instruction), expected);
    }

    #[test]
    fn test_display_instruction_empty() {
        let instruction = Instruction::EMPTY;

        let expected = "";
        assert_eq!(format!("{}", instruction), expected);
    }
}
