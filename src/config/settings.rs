// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use std::{cmp, fmt, path::PathBuf};
use toml_edit::{
    visit::{visit_inline_table, visit_table_like_kv, Visit},
    Array, InlineTable, Item, Key, Table, Value,
};

/// Serialize and deserialize configuration settings.
pub trait Settings: cmp::PartialEq + fmt::Debug + From<(Key, Item)> {
    fn to_toml(&self) -> (Key, Item);
}

/// Repository configuration settings.
///
/// Intermediary structure meant to help make it easier to deserialize and
/// serialize repository configuration file data.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct RepoSettings {
    /// Name of repository.
    pub name: String,

    /// Default branch.
    pub branch: String,

    /// Default remote.
    pub remote: String,

    /// Flag to determine if repository's working directory is the user's home
    /// directory through _bare_ technique.
    pub workdir_home: bool,

    /// Bootstrap configuration for repository.
    pub bootstrap: Option<BootstrapSettings>,
}

impl RepoSettings {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            branch: Default::default(),
            remote: Default::default(),
            workdir_home: Default::default(),
            bootstrap: Default::default(),
        }
    }

    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = branch.into();
        self
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.remote = remote.into();
        self
    }

    pub fn workdir_home(mut self, choice: bool) -> Self {
        self.workdir_home = choice;
        self
    }

    pub fn bootstrap(mut self, bootstrap: BootstrapSettings) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }
}

impl Settings for RepoSettings {
    fn to_toml(&self) -> (Key, Item) {
        let mut repo = Table::new();
        let mut repo_bootstrap = Table::new();

        repo.insert("branch", Item::Value(Value::from(&self.branch)));
        repo.insert("remote", Item::Value(Value::from(&self.remote)));
        repo.insert("workdir_home", Item::Value(Value::from(self.workdir_home)));
        if let Some(bootstrap) = &self.bootstrap {
            if let Some(clone) = &bootstrap.clone {
                repo_bootstrap.insert("clone", Item::Value(Value::from(clone)));
            }
            if let Some(os) = &bootstrap.os {
                repo_bootstrap.insert("os", Item::Value(Value::from(os.to_string())));
            }
            if let Some(users) = &bootstrap.users {
                repo_bootstrap.insert("users", Item::Value(Value::Array(Array::from_iter(users))));
            }
            if let Some(hosts) = &bootstrap.hosts {
                repo_bootstrap.insert("hosts", Item::Value(Value::Array(Array::from_iter(hosts))));
            }
            repo.insert("bootstrap", Item::Table(repo_bootstrap));
        }

        let key = Key::new(&self.name);
        let value = Item::Table(repo);
        (key, value)
    }
}

fn repo_toml<'toml>(entry: (&'toml Key, &'toml Item)) -> RepoSettings {
    let (key, value) = entry;
    let mut bootstrap = BootstrapSettings::new();
    let mut repo = RepoSettings::new(key.get());
    bootstrap.visit_item(value);
    repo.visit_item(value);

    // INVARIANT: if all bootstrap fields are None, then make the boostrap field itself None.
    if !bootstrap.is_empty() {
        repo = repo.bootstrap(bootstrap);
    }

    repo
}

impl<'toml> From<(&'toml Key, &'toml Item)> for RepoSettings {
    fn from(entry: (&'toml Key, &'toml Item)) -> RepoSettings {
        repo_toml(entry)
    }
}

impl From<(Key, Item)> for RepoSettings {
    fn from(entry: (Key, Item)) -> Self {
        let (key, value) = entry;
        repo_toml((&key, &value))
    }
}

impl<'toml> Visit<'toml> for RepoSettings {
    fn visit_table_like_kv(&mut self, key: &'toml str, node: &'toml Item) {
        match key {
            "branch" => self.branch = node.as_str().unwrap_or_default().to_string(),
            "remote" => self.remote = node.as_str().unwrap_or_default().to_string(),
            "workdir_home" => self.workdir_home = node.as_bool().unwrap_or_default(),
            &_ => visit_table_like_kv(self, key, node),
        }
        visit_table_like_kv(self, key, node);
    }
}

/// Repository bootstrap configuration settings.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct BootstrapSettings {
    /// URL to clone repository from.
    pub clone: Option<String>,

    /// Bootstrap repository if and only if user is using a specific OS.
    pub os: Option<OsType>,

    /// Bootstrap repository if and only if user is logged on to a specific
    /// set of user accounts.
    pub users: Option<Vec<String>>,

    /// Bootstrap repository if and only if user is logged on to a specific
    /// set of hosts.
    pub hosts: Option<Vec<String>>,
}

impl BootstrapSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clone(mut self, url: impl Into<String>) -> Self {
        self.clone = Some(url.into());
        self
    }

    pub fn os(mut self, os: OsType) -> Self {
        self.os = Some(os);
        self
    }

    pub fn users<I, S>(mut self, users: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut vec = Vec::new();
        vec.extend(users.into_iter().map(Into::into));
        self.users = Some(vec);
        self
    }

    pub fn hosts<I, S>(mut self, hosts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut vec = Vec::new();
        vec.extend(hosts.into_iter().map(Into::into));
        self.hosts = Some(vec);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.clone.is_none() && self.os.is_none() && self.users.is_none() && self.hosts.is_none()
    }
}

impl<'toml> Visit<'toml> for BootstrapSettings {
    fn visit_table_like_kv(&mut self, key: &'toml str, node: &'toml Item) {
        match key {
            "clone" => {
                if let Some(clone) = node.as_str() {
                    self.clone = Some(clone.to_string())
                }
            }
            "os" => {
                if let Some(os) = node.as_str() {
                    self.os = Some(OsType::from(os))
                }
            }
            "users" => {
                if let Some(users) = node.as_array() {
                    let data = users
                        .into_iter()
                        .map(|s| {
                            s.as_str().unwrap().trim_matches(|c| c == '\"' || c == '\'').to_string()
                        })
                        .collect();
                    self.users = Some(data)
                }
            }
            "hosts" => {
                if let Some(hosts) = node.as_array() {
                    let data = hosts
                        .into_iter()
                        .map(|s| {
                            s.as_str().unwrap().trim_matches(|c| c == '\"' || c == '\'').to_string()
                        })
                        .collect();
                    self.hosts = Some(data)
                }
            }
            &_ => visit_table_like_kv(self, key, node),
        }
        visit_table_like_kv(self, key, node);
    }
}

/// Operating System settings.
///
/// Simple enum used to determine the target OS user wants to bootstrap with.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum OsType {
    /// Bootstrap to any operating system.
    #[default]
    Any,

    /// Bootstrap to Unix-like systems only.
    Unix,

    /// Bootstrap to MacOS systems only.
    MacOs,

    /// Bootstrap to Windows system only.
    Windows,
}

impl From<&str> for OsType {
    fn from(data: &str) -> Self {
        match data {
            "any" => Self::Any,
            "unix" => Self::Unix,
            "macos" => Self::MacOs,
            "windows" => Self::Windows,
            &_ => Self::Any,
        }
    }
}

impl fmt::Display for OsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsType::Any => write!(f, "any"),
            OsType::Unix => write!(f, "unix"),
            OsType::MacOs => write!(f, "macos"),
            OsType::Windows => write!(f, "windows"),
        }
    }
}

/// Command hook settings.
///
/// An intermediary structure to help deserialize and serialize command hook
/// from Ricer's command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CmdHookSettings {
    /// Name of command to bind hook definitions too.
    pub cmd: String,

    /// Array of hook definitions to execute.
    pub hooks: Vec<HookSettings>,
}

impl CmdHookSettings {
    pub fn new(cmd: impl Into<String>) -> Self {
        Self { cmd: cmd.into(), hooks: Default::default() }
    }

    pub fn add_hook(mut self, hook: HookSettings) -> Self {
        self.hooks.push(hook);
        self
    }
}

impl Settings for CmdHookSettings {
    fn to_toml(&self) -> (Key, Item) {
        let mut tables = Array::new();
        let mut iter = self.hooks.iter().enumerate().peekable();
        while let Some((_, hook)) = iter.next() {
            let mut inline = InlineTable::new();
            let decor = inline.decor_mut();

            // INVARIANT: inline tables in array must be indented by 4 spaces.
            decor.set_prefix("\n    ");

            // INVARIANT: array ending delimiter ']' must be on its own line.
            if iter.peek().is_none() {
                decor.set_suffix("\n");
            }

            if let Some(pre) = &hook.pre {
                inline.insert("pre", Value::from(pre));
            }

            if let Some(post) = &hook.post {
                inline.insert("post", Value::from(post));
            }

            if let Some(workdir) = &hook.workdir {
                inline.insert("workdir", Value::from(String::from(workdir.to_string_lossy())));
            }

            tables.push_formatted(Value::from(inline));
        }

        let key = Key::new(&self.cmd);
        let value = Item::Value(Value::from(tables));
        (key, value)
    }
}

fn from_toml<'toml>(entry: (&'toml Key, &'toml Item)) -> CmdHookSettings {
    let (key, value) = entry;
    let mut cmd_hook = CmdHookSettings::new(key.get());
    cmd_hook.visit_item(value);
    cmd_hook
}

impl<'toml> From<(&'toml Key, &'toml Item)> for CmdHookSettings {
    fn from(entry: (&'toml Key, &'toml Item)) -> Self {
        from_toml(entry)
    }
}

impl From<(Key, Item)> for CmdHookSettings {
    fn from(entry: (Key, Item)) -> Self {
        let (key, value) = entry;
        from_toml((&key, &value))
    }
}

impl<'toml> Visit<'toml> for CmdHookSettings {
    fn visit_inline_table(&mut self, node: &'toml InlineTable) {
        let hook = HookSettings {
            pre: node.get("pre").and_then(|s| s.as_str().map(|s| s.into())),
            post: node.get("post").and_then(|s| s.as_str().map(|s| s.into())),
            workdir: node.get("workdir").and_then(|s| s.as_str().map(|s| s.into())),
        };
        self.hooks.push(hook);
        visit_inline_table(self, node);
    }
}

/// Hook definition settings.
///
/// An intermediary structure to help deserialize and serialize hook entries
/// for command hook settings in command hook configuration file.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HookSettings {
    /// Execute hook script _before_ command itself.
    pub pre: Option<String>,

    /// Execute hook script _after_ command itself.
    pub post: Option<String>,

    /// Set working directory of hook script.
    pub workdir: Option<PathBuf>,
}

impl HookSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pre(mut self, script: impl Into<String>) -> Self {
        self.pre = Some(script.into());
        self
    }

    pub fn post(mut self, script: impl Into<String>) -> Self {
        self.post = Some(script.into());
        self
    }

    pub fn workdir(mut self, path: impl Into<PathBuf>) -> Self {
        self.workdir = Some(path.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use toml_edit::DocumentMut;

    #[fixture]
    fn repo_settings_doc() -> Result<DocumentMut> {
        let doc: DocumentMut = indoc! {r#"
            [foo]
            branch = "master"
            remote = "origin"
            workdir_home = true

            [bar]
            branch = "main"
            remote = "origin"
            workdir_home = false

            [bar.bootstrap]
            clone = "https://some/url"
            os = "unix"
            users = ["awkless", "sedgwick"]
            hosts = ["lovelace", "turing"]
        "#}
        .parse()?;
        Ok(doc)
    }

    #[fixture]
    fn cmd_hook_settings_doc() -> Result<DocumentMut> {
        let doc: DocumentMut = indoc! {r#"
            commit = [
                { pre = "hook.sh", post = "hook.sh", workdir = "/some/path" },
                { pre = "hook.sh" },
                { post = "hook.sh" }
            ]
        "#}
        .parse()?;
        Ok(doc)
    }

    #[rstest]
    #[case::no_bootstrap(
        RepoSettings::new("foo")
            .branch("master")
            .remote("origin")
            .workdir_home(true),
    )]
    #[case::with_bootstrap(
        RepoSettings::new("bar")
            .branch("main")
            .remote("origin")
            .workdir_home(false)
            .bootstrap(
                BootstrapSettings::new()
                    .clone("https://some/url")
                    .os(OsType::Unix)
                    .users(["awkless", "sedgwick"])
                    .hosts(["lovelace", "turing"])
            ),
    )]
    fn repo_settings_from_key_item_return_self(
        repo_settings_doc: Result<DocumentMut>,
        #[case] expect: RepoSettings,
    ) -> Result<()> {
        let result = RepoSettings::from(
            repo_settings_doc?.as_table().get_key_value(expect.name.as_str()).unwrap(),
        );
        assert_eq!(result, expect);
        Ok(())
    }

    #[rstest]
    #[case::no_bootstrap(
        RepoSettings::new("foo")
            .branch("master")
            .remote("origin")
            .workdir_home(true),
        indoc! {r#"
            [foo]
            branch = "master"
            remote = "origin"
            workdir_home = true
        "#},
    )]
    #[case::with_bootstrap(
        RepoSettings::new("bar")
            .branch("main")
            .remote("origin")
            .workdir_home(false)
            .bootstrap(
                BootstrapSettings::new()
                    .clone("https://some/url")
                    .os(OsType::Unix)
                    .users(["awkless", "sedgwick"])
                    .hosts(["lovelace", "turing"])
            ),
        indoc! {r#"
            [bar]
            branch = "main"
            remote = "origin"
            workdir_home = false

            [bar.bootstrap]
            clone = "https://some/url"
            os = "unix"
            users = ["awkless", "sedgwick"]
            hosts = ["lovelace", "turing"]
        "#},
    )]
    fn repo_settings_to_toml_return_key_item(
        #[case] input: RepoSettings,
        #[case] expect: &str,
    ) -> Result<()> {
        let (key, item) = input.to_toml();
        let mut doc = DocumentMut::new();
        let table = doc.as_table_mut();
        table.insert_formatted(&key, item);
        table.set_implicit(true);
        assert_eq!(doc.to_string(), expect);
        Ok(())
    }

    #[rstest]
    #[case(
        CmdHookSettings::new("commit")
            .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/path"))
            .add_hook(HookSettings::new().pre("hook.sh"))
            .add_hook(HookSettings::new().post("hook.sh")),
    )]
    fn cmd_hook_settings_from_key_item_return_self(
        cmd_hook_settings_doc: Result<DocumentMut>,
        #[case] expect: CmdHookSettings,
    ) -> Result<()> {
        let result = CmdHookSettings::from(
            cmd_hook_settings_doc?.as_table().get_key_value(expect.cmd.as_str()).unwrap(),
        );
        assert_eq!(result, expect);
        Ok(())
    }

    #[rstest]
    #[case(
        CmdHookSettings::new("commit")
            .add_hook(HookSettings::new().pre("hook.sh").post("hook.sh").workdir("/some/path"))
            .add_hook(HookSettings::new().pre("hook.sh"))
            .add_hook(HookSettings::new().post("hook.sh")),
        indoc! {r#"
            commit = [
                { pre = "hook.sh", post = "hook.sh", workdir = "/some/path" },
                { pre = "hook.sh" },
                { post = "hook.sh" }
            ]
        "#},
    )]
    fn cmd_hook_settings_to_toml_return_key_item(
        #[case] input: CmdHookSettings,
        #[case] expect: &str,
    ) -> Result<()> {
        let (key, item) = input.to_toml();
        let mut doc = DocumentMut::new();
        let table = doc.as_table_mut();
        table.insert_formatted(&key, item);
        table.set_implicit(true);
        assert_eq!(doc.to_string(), expect);
        Ok(())
    }
}
