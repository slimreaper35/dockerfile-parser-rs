#[derive(Debug, PartialEq, Clone)]
/// Represents a Dockerfile instruction.
///
/// See: https://docs.docker.com/reference/dockerfile/#overview
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
    Cmd {
        command: String,
    },
    Copy {
        from: Option<String>,
        chown: Option<String>,
        chmod: Option<String>,
        link: Option<String>,
        sources: Vec<String>,
        destination: String,
    },
    Entrypoint {
        command: String,
    },
    Env {
        key: String,
        value: String,
    },
    Expose {
        port: usize,
    },
    From {
        platform: Option<String>,
        image: String,
        alias: Option<String>,
    },
    Label {
        key: String,
        value: String,
    },
    Run {
        mount: Option<String>,
        network: Option<String>,
        security: Option<String>,
        command: String,
    },
    User {
        user: String,
        group: Option<String>,
    },
    Volume {
        mount: String,
    },
    Workdir {
        path: String,
    },
    //-------------//
    //    Extra    //
    Comment(String),
    Empty,
}
