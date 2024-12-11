use crate::transport::{ReadStream, Transport, WriteStream};
use crate::types::JsonRpcMessage;
use async_trait::async_trait;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

pub struct StdioServerParams {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
}

pub struct StdioTransport {
    pub params: StdioServerParams,
}

impl StdioTransport {
    fn get_default_environment() -> std::collections::HashMap<String, String> {
        let default_vars = if cfg!(windows) {
            vec!["APPDATA", "PATH", "TEMP", "USERNAME"] // Simplified list
        } else {
            vec!["HOME", "PATH", "SHELL", "USER"] // Simplified list
        };

        std::env::vars()
            .filter(|(key, value)| default_vars.contains(&key.as_str()) && !value.starts_with("()"))
            .collect()
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn connect(
        &self,
    ) -> Result<(ReadStream, WriteStream), Box<dyn std::error::Error + Send>> {
        let mut child = Command::new(&self.params.command)
            .args(&self.params.args)
            .env_clear()
            .envs(
                self.params
                    .env
                    .clone()
                    .unwrap_or_else(|| Self::get_default_environment()),
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

        let stdin = child.stdin.take().expect("Failed to get stdin");
        let stdout = child.stdout.take().expect("Failed to get stdout");

        let (tx_read, rx_read) = mpsc::channel(100);
        let (tx_write, mut rx_write) = mpsc::channel(100);

        // Spawn stdout reader task
        let stdout_reader = BufReader::new(stdout);
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                match serde_json::from_str::<JsonRpcMessage>(&line) {
                    Ok(msg) => {
                        if tx_read.send(Ok(msg)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx_read
                            .send(Err(Box::new(e) as Box<dyn std::error::Error + Send>))
                            .await;
                    }
                }
            }
        });

        // Spawn stdin writer task
        let mut stdin = stdin;
        tokio::spawn(async move {
            while let Some(message) = rx_write.recv().await {
                let json = serde_json::to_string(&message).expect("Failed to serialize message");
                if stdin
                    .write_all(format!("{}\n", json).as_bytes())
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        Ok((rx_read, tx_write))
    }
}
