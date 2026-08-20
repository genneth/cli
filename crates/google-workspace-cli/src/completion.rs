// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Shell completion script generation.

use clap::{Arg, Command};
use clap_complete::{Generator, Shell};
use futures_util::future::join_all;
use std::future::Future;

use crate::discovery::RestDescription;
use crate::error::GwsError;
use crate::services::{ServiceEntry, SERVICES};

/// Build the parser for `gws completion`.
fn completion_command() -> Command {
    Command::new("completion")
        .about("Generate a shell completion script")
        .arg(
            Arg::new("shell")
                .help("Shell to generate completions for")
                .required(true)
                .value_parser(clap::builder::EnumValueParser::<Shell>::new()),
        )
}

/// Handle `gws completion <shell>`.
pub async fn handle_completion_command(args: &[String]) -> Result<(), GwsError> {
    let Some(shell) = parse_completion_args(args)? else {
        return Ok(());
    };
    let mut command = build_completion_cli().await?;
    write_completion_script(shell, &mut command, &mut std::io::stdout())
}

fn parse_completion_args(args: &[String]) -> Result<Option<Shell>, GwsError> {
    let matches = match completion_command()
        .try_get_matches_from(std::iter::once("completion".to_string()).chain(args.iter().cloned()))
    {
        Ok(matches) => matches,
        Err(error)
            if error.kind() == clap::error::ErrorKind::DisplayHelp
                || error.kind() == clap::error::ErrorKind::DisplayVersion =>
        {
            error.print().map_err(|io_error| {
                GwsError::Validation(format!("Failed to print help: {io_error}"))
            })?;
            return Ok(None);
        }
        Err(error) => return Err(GwsError::Validation(error.to_string())),
    };

    Ok(matches.get_one::<Shell>("shell").copied())
}

fn write_completion_script(
    shell: Shell,
    command: &mut Command,
    output: &mut dyn std::io::Write,
) -> Result<(), GwsError> {
    command.set_bin_name("gws");
    command.build();

    shell
        .try_generate(command, output)
        .map_err(|error| GwsError::Other(anyhow::anyhow!("Failed to write completions: {error}")))
}

/// Build an aggregate command tree containing all known services.
async fn build_completion_cli() -> Result<Command, GwsError> {
    build_completion_cli_with(fetch_service_document).await
}

async fn build_completion_cli_with<F, Fut>(fetch: F) -> Result<Command, GwsError>
where
    F: Fn(&'static ServiceEntry) -> Fut,
    Fut: Future<Output = Result<RestDescription, GwsError>>,
{
    let documents = join_all(SERVICES.iter().map(fetch)).await;
    let mut command = base_command();

    for (entry, document) in SERVICES.iter().zip(documents) {
        command = add_service(command, entry, &document?);
    }

    Ok(command)
}

async fn fetch_service_document(entry: &ServiceEntry) -> Result<RestDescription, GwsError> {
    if entry.api_name == "workflow" {
        return Ok(RestDescription {
            name: "workflow".to_string(),
            title: Some("Workflow".to_string()),
            description: Some(entry.description.to_string()),
            ..Default::default()
        });
    }

    crate::discovery::fetch_discovery_document(entry.api_name, entry.version)
        .await
        .map_err(|error| {
            GwsError::Discovery(format!(
                "Failed to load completions for {}: {error:#}",
                entry.aliases[0]
            ))
        })
}

fn base_command() -> Command {
    crate::commands::add_api_version_arg(
        Command::new("gws")
            .about("Google Workspace CLI")
            .version(env!("CARGO_PKG_VERSION"))
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(completion_command())
            .subcommand(schema_command())
            .subcommand(generate_skills_command())
            .subcommand(crate::auth_commands::auth_command()),
    )
}

fn schema_command() -> Command {
    Command::new("schema")
        .about("Inspect an API method's request and response schemas")
        .arg(
            Arg::new("method")
                .help("Method path, for example drive.files.list")
                .required(true),
        )
        .arg(
            Arg::new("resolve-refs")
                .long("resolve-refs")
                .help("Resolve schema references inline")
                .action(clap::ArgAction::SetTrue),
        )
}

fn generate_skills_command() -> Command {
    Command::new("generate-skills")
        .about("Generate agent skills from the gws command tree")
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .value_name("DIR")
                .default_value("skills"),
        )
        .arg(Arg::new("filter").long("filter").value_name("MATCH"))
        .arg(
            Arg::new("really-all")
                .long("really-all")
                .action(clap::ArgAction::SetTrue),
        )
}

fn add_service(command: Command, entry: &'static ServiceEntry, doc: &RestDescription) -> Command {
    let service =
        crate::commands::add_global_args(crate::commands::build_service_cli(entry.aliases[0], doc))
            .visible_aliases(entry.aliases.iter().skip(1).copied());
    command.subcommand(service)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::{RestMethod, RestResource};
    use std::collections::HashMap;

    static TEST_SERVICE: ServiceEntry = ServiceEntry {
        aliases: &["drive", "storage"],
        api_name: "drive",
        version: "v3",
        description: "Test Drive service",
    };

    fn test_document() -> RestDescription {
        let methods = HashMap::from([(
            "list".to_string(),
            RestMethod {
                http_method: "GET".to_string(),
                path: "files".to_string(),
                ..Default::default()
            },
        )]);
        let resources = HashMap::from([(
            "files".to_string(),
            RestResource {
                methods,
                ..Default::default()
            },
        )]);

        RestDescription {
            name: "drive".to_string(),
            description: Some("Test Drive service".to_string()),
            resources,
            ..Default::default()
        }
    }

    #[test]
    fn completion_command_accepts_supported_shell() {
        let matches = completion_command()
            .try_get_matches_from(["completion", "bash"])
            .unwrap();

        assert_eq!(matches.get_one::<Shell>("shell"), Some(&Shell::Bash));
    }

    #[test]
    fn completion_command_rejects_unknown_shell() {
        let error = completion_command()
            .try_get_matches_from(["completion", "unknown"])
            .unwrap_err();

        assert_eq!(error.kind(), clap::error::ErrorKind::InvalidValue);
    }

    #[test]
    fn aggregate_tree_contains_resources_helpers_and_aliases() {
        let command = add_service(base_command(), &TEST_SERVICE, &test_document());
        let drive = command
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == "drive")
            .unwrap();

        assert!(drive.get_visible_aliases().any(|name| name == "storage"));
        assert!(drive
            .get_subcommands()
            .any(|subcommand| subcommand.get_name() == "+upload"));
        assert!(drive
            .get_arguments()
            .any(|argument| argument.get_id() == "format"));
        let files = drive
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == "files")
            .unwrap();
        assert!(files
            .get_subcommands()
            .any(|subcommand| subcommand.get_name() == "list"));
        let auth = command
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == "auth")
            .unwrap();
        assert!(!auth
            .get_arguments()
            .any(|argument| argument.get_id() == "format"));
    }

    #[test]
    fn generated_bash_script_contains_service_and_method() {
        let mut command = add_service(base_command(), &TEST_SERVICE, &test_document());
        let mut output = Vec::new();

        write_completion_script(Shell::Bash, &mut command, &mut output).unwrap();
        let script = String::from_utf8(output).unwrap();

        assert!(script.contains("drive"));
        assert!(script.contains("files"));
        assert!(script.contains("list"));
    }

    #[test]
    fn parser_returns_none_for_help() {
        assert_eq!(
            parse_completion_args(&["--help".to_string()]).unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn handler_rejects_unknown_shell_without_loading_discovery() {
        let error = handle_completion_command(&["unknown".to_string()])
            .await
            .unwrap_err();

        assert!(matches!(error, GwsError::Validation(_)));
    }

    #[tokio::test]
    async fn aggregate_builder_loads_every_known_service() {
        let command = build_completion_cli_with(|entry| async move {
            Ok(RestDescription {
                name: entry.api_name.to_string(),
                description: Some(entry.description.to_string()),
                ..Default::default()
            })
        })
        .await
        .unwrap();

        for entry in SERVICES {
            assert!(command
                .get_subcommands()
                .any(|subcommand| subcommand.get_name() == entry.aliases[0]));
        }
    }

    #[tokio::test]
    async fn aggregate_builder_propagates_discovery_errors() {
        let error = build_completion_cli_with(|entry| async move {
            if entry.api_name == "drive" {
                Err(GwsError::Discovery("test failure".to_string()))
            } else {
                Ok(RestDescription {
                    name: entry.api_name.to_string(),
                    ..Default::default()
                })
            }
        })
        .await
        .unwrap_err();

        assert!(matches!(error, GwsError::Discovery(_)));
    }

    #[tokio::test]
    async fn synthetic_workflow_document_does_not_require_discovery() {
        let entry = SERVICES
            .iter()
            .find(|entry| entry.api_name == "workflow")
            .unwrap();
        let document = fetch_service_document(entry).await.unwrap();

        assert_eq!(document.name, "workflow");
        assert_eq!(document.title.as_deref(), Some("Workflow"));
    }
}
