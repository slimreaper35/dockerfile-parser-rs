// https://docs.docker.com/reference/dockerfile/#overview

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
/// This enum represents available instructions in a Dockerfile and their associated data.
pub enum Instruction {
    /// Represents an ADD instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let add = Instruction::Add {
    ///     checksum: None,
    ///     chown: None,
    ///     chmod: None,
    ///     link: None,
    ///     sources: Vec::from([String::from("source1"), String::from("source2")]),
    ///     destination: String::from("/destination"),
    /// };
    /// ```
    Add {
        checksum: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    /// Represents an ARG instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    ///
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let arg = Instruction::Arg(BTreeMap::from([
    ///     (String::from("ARG1"), Some(String::from("value1"))),
    ///     (String::from("ARG2"), None),
    /// ]));
    /// ```
    Arg(BTreeMap<String, Option<String>>),
    /// Represents a CMD instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let cmd = Instruction::Cmd(Vec::from([
    ///     String::from("echo"),
    ///     String::from("Hello, World!"),
    /// ]));
    /// ```
    Cmd(Vec<String>),
    /// Represents a comment in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let comment = Instruction::Comment(String::from("# This is a comment"));
    /// ```
    Comment(String),
    /// Represents a COPY instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let copy = Instruction::Copy {
    ///     from: Some(String::from("builder")),
    ///     chown: None,
    ///     chmod: None,
    ///     link: None,
    ///     sources: Vec::from([String::from("source1"), String::from("source2")]),
    ///     destination: String::from("/destination"),
    /// };
    /// ```
    Copy {
        from: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    /// Represents an empty line in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let empty = Instruction::Empty;
    /// ```
    Empty,
    /// Represents an ENTRYPOINT instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let entrypoint = Instruction::Entrypoint(Vec::from([String::from("entrypoint.sh")]));
    /// ```
    Entrypoint(Vec<String>),
    /// Represents an ENV instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    ///
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let env = Instruction::Env(BTreeMap::from([
    ///     (String::from("ENV1"), String::from("value1")),
    ///     (String::from("ENV2"), String::from("value2")),
    /// ]));
    /// ```
    Env(BTreeMap<String, String>),
    /// Represents an EXPOSE instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let expose = Instruction::Expose {
    ///     ports: Vec::from([String::from("8080")]),
    /// };
    /// ```
    Expose { ports: Vec<String> },
    /// Represents a FROM instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let from = Instruction::From {
    ///     platform: Some(String::from("linux/amd64")),
    ///     image: String::from("docker.io/library/fedora:latest"),
    ///     alias: Some(String::from("builder")),
    /// };
    /// ```
    From {
        platform: Option<String>,
        image: String,
        alias: Option<String>,
    },
    /// Represents a LABEL instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    ///
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let label = Instruction::Label(BTreeMap::from([
    ///     (String::from("version"), String::from("1.0")),
    ///     (String::from("maintainer"), String::from("John Doe")),
    /// ]));
    /// ```
    Label(BTreeMap<String, String>),
    /// Represents a RUN instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let run = Instruction::Run {
    ///     mount: None,
    ///     network: None,
    ///     security: None,
    ///     command: Vec::from([String::from("<<EOF")]),
    ///     heredoc: Some(Vec::from([
    ///         String::from("dnf upgrade -y"),
    ///         String::from("dnf install -y rustup"),
    ///         String::from("EOF"),
    ///     ])),
    /// };
    /// ```
    Run {
        mount: Option<String>,
        network: Option<String>,
        security: Option<String>,
        command: Vec<String>,
        heredoc: Option<Vec<String>>,
    },
    /// Represents a SHELL instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let shell = Instruction::Shell(Vec::from([String::from("/bin/sh"), String::from("-c")]));
    /// ```
    Shell(Vec<String>),
    /// Represents a STOPSIGNAL instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let stopsignal = Instruction::Stopsignal {
    ///     signal: String::from("SIGTERM"),
    /// };
    /// ```
    Stopsignal { signal: String },
    /// Represents a USER instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let user = Instruction::User {
    ///     user: String::from("1001"),
    ///     group: None,
    /// };
    /// ```
    User { user: String, group: Option<String> },
    /// Represents a VOLUME instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let volume = Instruction::Volume {
    ///     mounts: Vec::from([String::from("/data")]),
    /// };
    /// ```
    Volume { mounts: Vec<String> },
    /// Represents a WORKDIR instruction in the Dockerfile.
    ///
    /// ### Example
    ///
    /// ```
    /// use dockerfile_parser_rs::Instruction;
    ///
    /// let workdir = Instruction::Workdir {
    ///     path: String::from("/app"),
    /// };
    /// ```
    Workdir { path: String },
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add {
                checksum,
                chown,
                chmod,
                link,
                sources,
                destination,
            } => {
                let options = vec![
                    helpers::format_instruction_option("checksum", checksum.as_ref()),
                    helpers::format_instruction_option("chown", chown.as_ref()),
                    helpers::format_instruction_option("chmod", chmod.as_ref()),
                    helpers::format_instruction_option("link", link.as_ref()),
                ];
                let prefix = helpers::format_options_string(&options);
                write!(f, "ADD {prefix}{} {destination}", sources.join(" "))
            }
            Self::Arg(args) => write!(f, "ARG {}", helpers::format_optional_btree_map(args)),
            Self::Cmd(cmd) => write!(f, "CMD {cmd:?}"),
            Self::Comment(comment) => write!(f, "{comment}"),
            Self::Copy {
                from,
                chown,
                chmod,
                link,
                sources,
                destination,
            } => {
                let options = vec![
                    helpers::format_instruction_option("from", from.as_ref()),
                    helpers::format_instruction_option("chown", chown.as_ref()),
                    helpers::format_instruction_option("chmod", chmod.as_ref()),
                    helpers::format_instruction_option("link", link.as_ref()),
                ];
                let prefix = helpers::format_options_string(&options);
                write!(f, "COPY {prefix}{} {destination}", sources.join(" "))
            }
            Self::Empty => write!(f, ""),
            Self::Entrypoint(entrypoint) => write!(f, "ENTRYPOINT {entrypoint:?}"),
            Self::Env(env) => write!(f, "ENV {}", helpers::format_btree_map(env)),
            Self::Expose { ports } => write!(f, "EXPOSE {}", ports.join(" ")),
            Self::From {
                platform,
                image,
                alias,
            } => {
                let options = vec![helpers::format_instruction_option(
                    "platform",
                    platform.as_ref(),
                )];
                let prefix = helpers::format_options_string(&options);

                let mut line = format!("FROM {prefix}{image}");
                if let Some(alias) = alias {
                    line.push_str(" AS ");
                    line.push_str(alias);
                }
                write!(f, "{line}")
            }
            Self::Label(labels) => write!(f, "LABEL {}", helpers::format_btree_map(labels)),
            Self::Run {
                mount,
                network,
                security,
                command,
                heredoc,
            } => {
                let options = vec![
                    helpers::format_instruction_option("mount", mount.as_ref()),
                    helpers::format_instruction_option("network", network.as_ref()),
                    helpers::format_instruction_option("security", security.as_ref()),
                ];
                let prefix = helpers::format_options_string(&options);
                match heredoc {
                    Some(heredoc) => write!(
                        f,
                        "RUN {prefix}{}\n{}",
                        command.join(" "),
                        heredoc.join("\n")
                    ),
                    None => write!(f, "RUN {prefix}{command:?}"),
                }
            }
            Self::Shell(shell) => write!(f, "SHELL {shell:?}"),
            Self::Stopsignal { signal } => write!(f, "STOPSIGNAL {signal}"),
            Self::User { user, group } => match group {
                Some(group) => write!(f, "USER {user}:{group}"),
                None => write!(f, "USER {user}"),
            },
            Self::Volume { mounts } => write!(f, "VOLUME {mounts:?}"),
            Self::Workdir { path } => write!(f, "WORKDIR {path}"),
        }
    }
}

mod helpers {
    use std::collections::BTreeMap;

    use crate::quoter::Quoter;

    pub fn format_instruction_option(key: &str, value: Option<&String>) -> String {
        value
            .as_ref()
            .map(|v| format!("--{key}={v}"))
            .unwrap_or_default()
    }

    pub fn format_options_string(options: &[String]) -> String {
        let result = options
            .iter()
            .filter(|s| !s.is_empty())
            .cloned()
            .collect::<Vec<String>>()
            .join(" ");

        if result.is_empty() {
            String::new()
        } else {
            // add a space to separate options
            format!("{result} ")
        }
    }

    pub fn format_btree_map(pairs: &BTreeMap<String, String>) -> String {
        pairs
            .iter()
            .map(|(key, value)| format!("{key}={}", value.enquote()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn format_optional_btree_map(pairs: &BTreeMap<String, Option<String>>) -> String {
        pairs
            .iter()
            .map(|(k, v)| v.as_ref().map_or_else(|| k.clone(), |v| format!("{k}={v}")))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_instruction_add() {
        let instruction = Instruction::Add {
            checksum: None,
            chown: None,
            chmod: None,
            link: None,
            sources: vec![String::from("source1"), String::from("source2")],
            destination: String::from("/destination"),
        };

        let expected = "ADD source1 source2 /destination";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_arg() {
        let instruction = Instruction::Arg(BTreeMap::from([
            (String::from("ARG2"), None),
            (String::from("ARG1"), Some(String::from("value1"))),
        ]));

        // must be sorted
        let expected = "ARG ARG1=value1 ARG2";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_cmd() {
        let instruction =
            Instruction::Cmd(vec![String::from("echo"), String::from("Hello, World!")]);

        let expected = "CMD [\"echo\", \"Hello, World!\"]";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_copy() {
        let instruction = Instruction::Copy {
            from: Some(String::from("builder")),
            chown: None,
            chmod: None,
            link: None,
            sources: vec![String::from("source1"), String::from("source2")],
            destination: String::from("/destination"),
        };

        let expected = "COPY --from=builder source1 source2 /destination";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_entrypoint() {
        let instruction = Instruction::Entrypoint(vec![String::from("entrypoint.sh")]);

        let expected = "ENTRYPOINT [\"entrypoint.sh\"]";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_env() {
        let instruction = Instruction::Env(BTreeMap::from([
            (String::from("ENV2"), String::from("value2")),
            (String::from("ENV1"), String::from("value1")),
        ]));

        // must be sorted
        let expected = "ENV ENV1=\"value1\" ENV2=\"value2\"";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_expose() {
        let instruction = Instruction::Expose {
            ports: vec![String::from("80"), String::from("443")],
        };

        let expected = "EXPOSE 80 443";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_from() {
        let instruction = Instruction::From {
            platform: Some(String::from("linux/amd64")),
            image: String::from("docker.io/library/fedora:latest"),
            alias: Some(String::from("builder")),
        };

        let expected = "FROM --platform=linux/amd64 docker.io/library/fedora:latest AS builder";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_label() {
        let instruction = Instruction::Label(BTreeMap::from([
            (String::from("version"), String::from("1.0")),
            (String::from("maintainer"), String::from("John Doe")),
        ]));

        // must be sorted
        let expected = "LABEL maintainer=\"John Doe\" version=\"1.0\"";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_run() {
        let instruction = Instruction::Run {
            mount: None,
            network: None,
            security: None,
            command: vec![String::from("cat"), String::from("/etc/os-release")],
            heredoc: None,
        };

        let expected = "RUN [\"cat\", \"/etc/os-release\"]";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_run_with_heredoc() {
        let instruction = Instruction::Run {
            mount: None,
            network: None,
            security: None,
            command: vec![String::from("<<EOF")],
            heredoc: Some(vec![
                String::from("dnf upgrade -y"),
                String::from("dnf install -y rustup"),
                String::from("EOF"),
            ]),
        };

        let expected = "RUN <<EOF\ndnf upgrade -y\ndnf install -y rustup\nEOF";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_run_with_heredoc_and_tabs() {
        let instruction = Instruction::Run {
            mount: None,
            network: None,
            security: None,
            command: vec![String::from("python"), String::from("<<EOF")],
            heredoc: Some(vec![
                String::from("def main():"),
                String::from("\tx = 42"),
                String::from("\tprint(x)"),
                String::from(""),
                String::from("main()"),
                String::from("EOF"),
            ]),
        };

        let expected = "RUN python <<EOF\ndef main():\n\tx = 42\n\tprint(x)\n\nmain()\nEOF";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_shell() {
        let instruction = Instruction::Shell(vec![String::from("/bin/sh"), String::from("-c")]);

        let expected = "SHELL [\"/bin/sh\", \"-c\"]";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_user() {
        let instruction = Instruction::User {
            user: String::from("root"),
            group: Some(String::from("root")),
        };

        let expected = "USER root:root";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_volume() {
        let instruction = Instruction::Volume {
            mounts: vec![String::from("/data"), String::from("/var/log")],
        };

        let expected = "VOLUME [\"/data\", \"/var/log\"]";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_workdir() {
        let instruction = Instruction::Workdir {
            path: String::from("/app"),
        };

        let expected = "WORKDIR /app";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_comment() {
        let instruction = Instruction::Comment(String::from("# This is a comment"));

        let expected = "# This is a comment";
        assert_eq!(instruction.to_string(), expected);
    }

    #[test]
    fn test_display_instruction_empty() {
        let instruction = Instruction::Empty;

        let expected = "";
        assert_eq!(instruction.to_string(), expected);
    }
}
