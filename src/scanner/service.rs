use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Service {
    Ftp,
    Ssh,
    Telnet,
    Smtp,
    Dns,
    Http,
    Pop3,
    Imap,
    Https,
    Smb,
    Mysql,
    Rdp,
    Postgresql,
    Redis,
    HttpProxy,
    Mongodb,
    Unknown,
}

impl Service {
    pub fn from_port(port: u16) -> Self {
        match port {
            21 => Service::Ftp,
            22 => Service::Ssh,
            23 => Service::Telnet,
            25 => Service::Smtp,
            53 => Service::Dns,
            80 => Service::Http,
            110 => Service::Pop3,
            143 => Service::Imap,
            443 => Service::Https,
            445 => Service::Smb,
            3306 => Service::Mysql,
            3389 => Service::Rdp,
            5432 => Service::Postgresql,
            6379 => Service::Redis,
            8080 => Service::HttpProxy,
            27017 => Service::Mongodb,
            _ => Service::Unknown,
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Service::Ftp => "FTP",
            Service::Ssh => "SSH",
            Service::Telnet => "TELNET",
            Service::Smtp => "SMTP",
            Service::Dns => "DNS",
            Service::Http => "HTTP",
            Service::Pop3 => "POP3",
            Service::Imap => "IMAP",
            Service::Https => "HTTPS",
            Service::Smb => "SMB",
            Service::Mysql => "MYSQL",
            Service::Rdp => "RDP",
            Service::Postgresql => "POSTGRESQL",
            Service::Redis => "REDIS",
            Service::HttpProxy => "HTTP-PROXY",
            Service::Mongodb => "MONGODB",
            Service::Unknown => "UNKNOWN",
        };
        write!(f, "{name}")
    }
}
