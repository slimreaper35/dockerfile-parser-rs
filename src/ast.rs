// https://docs.docker.com/reference/dockerfile/#overview

use std::collections::HashMap;
use std::fmt;

use strum_macros::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Instruction {
    Add {
        checksum: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    Arg {
        name: String,
        default: Option<String>,
    },
    Cmd(Vec<String>),
    Copy {
        from: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    Entrypoint(Vec<String>),
    Env(HashMap<String, String>),
    Expose {
        port: String,
        protocol: Option<Protocol>,
    },
    From {
        platform: Option<String>,
        image: String,
        alias: Option<String>,
    },
    Label(HashMap<String, String>),
    Run {
        mount: Option<String>,
        network: Option<String>,
        security: Option<String>,
        command: Vec<String>,
    },
    Shell(Vec<String>),
    User {
        user: String,
        group: Option<String>,
    },
    Volume {
        mounts: Vec<String>,
    },
    Workdir {
        path: String,
    },
    //-------------//
    //    Extra    //
    //-------------//
    Comment(String),
    Empty,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add {
                checksum,
                chown,
                chmod,
                link,
                sources,
                destination,
            } => {
                let options = vec![
                    helpers::format_option("checksum", checksum),
                    helpers::format_option("chown", chown),
                    helpers::format_option("chmod", chmod),
                    helpers::format_option("link", link),
                ];
                let options_string = helpers::format_multiple(options);

                let prefix = if options_string.is_empty() {
                    String::new()
                } else {
                    format!("{} ", options_string)
                };
                write!(f, "ADD {}{} {}", prefix, sources.join(" "), destination)
            }
            Instruction::Arg { name, default } => {
                if let Some(default) = default {
                    write!(f, "ARG {}={}", name, default)
                } else {
                    write!(f, "ARG {}", name)
                }
            }
            Instruction::Cmd(cmd) => write!(f, "CMD {:?}", cmd),
            Instruction::Copy {
                from,
                chown,
                chmod,
                link,
                sources,
                destination,
            } => {
                let options = vec![
                    helpers::format_option("from", from),
                    helpers::format_option("chown", chown),
                    helpers::format_option("chmod", chmod),
                    helpers::format_option("link", link),
                ];
                let options_string = helpers::format_multiple(options);
                write!(
                    f,
                    "COPY {}{} {}",
                    options_string,
                    sources.join(" "),
                    destination
                )
            }
            Instruction::Entrypoint(entrypoint) => write!(f, "ENTRYPOINT {:?}", entrypoint),
            Instruction::Env(env) => {
                let mut env_vars = Vec::new();
                for (key, value) in env {
                    env_vars.push(format!("{}=\"{}\"", key, value));
                }
                write!(f, "ENV {}", env_vars.join(" "))
            }
            Instruction::Expose { port, protocol } => {
                if let Some(protocol) = protocol {
                    write!(f, "EXPOSE {}/{:?}", port, protocol)
                } else {
                    write!(f, "EXPOSE {}", port)
                }
            }
            Instruction::From {
                platform,
                image,
                alias,
            } => {
                let mut line = String::new();
                if let Some(platform) = platform {
                    line.push_str(&format!("--platform={} ", platform));
                }
                line.push_str(image);
                if let Some(alias) = alias {
                    line.push_str(&format!(" AS {}", alias));
                }
                write!(f, "FROM {}", line)
            }
            Instruction::Label(labels) => {
                let mut label_pairs = Vec::new();
                for (key, value) in labels {
                    label_pairs.push(format!("{}=\"{}\"", key, value));
                }
                write!(f, "LABEL {}", label_pairs.join(" "))
            }
            Instruction::Run {
                mount,
                network,
                security,
                command,
            } => {
                let options = vec![
                    helpers::format_option("mount", mount),
                    helpers::format_option("network", network),
                    helpers::format_option("security", security),
                ];
                let options_string = helpers::format_multiple(options);
                write!(f, "RUN {}{:?}", options_string, command)
            }
            Instruction::Shell(shell) => write!(f, "SHELL {:?}", shell),
            Instruction::User { user, group } => {
                if let Some(group) = group {
                    write!(f, "USER {}:{}", user, group)
                } else {
                    write!(f, "USER {}", user)
                }
            }
            Instruction::Volume { mounts } => write!(f, "VOLUME {:?}", mounts),
            Instruction::Workdir { path } => write!(f, "WORKDIR {}", path),
            //-------------//
            //    Extra    //
            //-------------//
            Instruction::Comment(comment) => write!(f, "{}", comment),
            Instruction::Empty => write!(f, ""),
        }
    }
}

mod helpers {

    pub fn format_option(key: &str, value: &Option<String>) -> String {
        value
            .as_ref()
            .map(|v| format!("--{}={}", key, v))
            .unwrap_or_default()
    }

    pub fn format_multiple(options: Vec<String>) -> String {
        options
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
