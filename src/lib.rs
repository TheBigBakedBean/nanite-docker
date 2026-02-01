//! Nanite docker is an intermediate representation of a Dockerfile.
//! It intends to provide a strongly typed library for working with Dockerfiles in your code.
//! > Note: This is intended to be a low level library, intending to provide only the representation, and not any quality of life features.
//! > if you have a library that provides some of these features, please add a pull request and I will be happy to add those libraries here.
//!
//! # Examples
//! ## Basic Example
//! Note that this example lacks an assert_eq!, see the specific instruction documentation to see what each instruction renders as
//! ```rust
//! use nanite_docker::*;
//! let dockerfile = Dockerfile {
//!     config: None,
//!     comment: Some("Basic Dockerfile Example".into()),
//!     stages: vec![Stage{
//!         pre_from_instructions: vec![],
//!         from: From {
//!             platform: None,
//!             image: "ubuntu:latest".into(),
//!             alias: None,
//!         },
//!         instructions: vec![
//!             Instruction::Cmd(Cmd{
//!                 argv: vec!["echo".into(), "hello".into()]
//!             })
//!         ]
//!     }]
//! };
//!
//! let built_dockerfile = format!("{dockerfile}");
//! ```
//!
//! ## Long Example (Showcases most features)
//! ```rust
//! use nanite_docker::*;
//! fn main() {
//!     let dockerfile = Dockerfile {
//!         config: None,
//!         comment: Some("Full instruction coverage test scenario".into()),
//!         stages: vec![Stage {
//!             // ── Pre-FROM ─────────────────────────────────────────────
//!             pre_from_instructions: vec![
//!                 PreFromInstruction::Arg(Arg {
//!                     name: "BASE_IMAGE".into(),
//!                     default: Some("ubuntu:24.04".into()),
//!                 }),
//!                 PreFromInstruction::Label(Label {
//!                     key: "prefrom.label.example".into(),
//!                     value: "true".into(),
//!                 }),
//!             ],
//!
//!             from: From {
//!                 platform: None,
//!                 image: "$BASE_IMAGE".into(),
//!                 alias: None,
//!             },
//!
//!             // ── Instructions ─────────────────────────────────────────
//!             instructions: vec![
//!                 // MAINTAINER (legacy)
//!                 Instruction::Maintainer(Maintainer {
//!                     name: "Example Maintainer <maintainer@example.com>".into(),
//!                 }),
//!                 // ARG
//!                 Instruction::Arg(Arg {
//!                     name: "BUILD_MODE".into(),
//!                     default: Some("release".into()),
//!                 }),
//!                 // ENV
//!                 Instruction::Env(Env {
//!                     key: "RUST_LOG".into(),
//!                     value: "info".into(),
//!                 }),
//!                 // WORKDIR
//!                 Instruction::Workdir(WorkDir {
//!                     path: "/app".into(),
//!                 }),
//!                 // USER (name)
//!                 Instruction::User(User::ByName {
//!                     name: "root".into(),
//!                     group: None,
//!                 }),
//!                 // ADD
//!                 Instruction::Add(Add {
//!                     opts: vec![AddOpt::Chown {
//!                         user: "0".into(),
//!                         group: Some("0".into()),
//!                     }],
//!                     src: vec!["./archive.tar.gz".into()],
//!                     dest: "/app/".into(),
//!                 }),
//!                 // COPY
//!                 Instruction::Copy(Copy {
//!                     opts: vec![CopyOpt::Chown {
//!                         user: "0".into(),
//!                         group: Some("0".into()),
//!                     }],
//!                     src: vec!["./src".into()],
//!                     dest: "/app/src".into(),
//!                 }),
//!                 // EXPOSE
//!                 Instruction::Expose(Expose {
//!                     port: 8080,
//!                     protocol: Some(ExposeProtocol::tcp),
//!                 }),
//!                 Instruction::Expose(Expose {
//!                     port: 9090,
//!                     protocol: Some(ExposeProtocol::udp),
//!                 }),
//!                 // LABEL
//!                 Instruction::Label(Label {
//!                     key: "app.name".into(),
//!                     value: "example".into(),
//!                 }),
//!                 // VOLUME
//!                 Instruction::Volume(Volume {
//!                     volumes: vec!["/var/lib/appdata".into()],
//!                 }),
//!                 // SHELL
//!                 Instruction::Shell(Shell {
//!                     argv: vec!["/bin/bash".into(), "-c".into()],
//!                 }),
//!                 // HEALTHCHECK
//!                 Instruction::HealthCheck(HealthCheck::Some {
//!                     opts: vec![
//!                         HealthCheckOpt::Interval("30s".into()),
//!                         HealthCheckOpt::Timeout("3s".into()),
//!                         HealthCheckOpt::Retries(3),
//!                     ],
//!                     cmd: Cmd {
//!                         argv: vec![
//!                             "curl".into(),
//!                             "-f".into(),
//!                             "http://localhost:8080/health".into(),
//!                             "||".into(),
//!                             "exit".into(),
//!                             "1".into(),
//!                         ],
//!                     },
//!                 }),
//!                 // STOPSIGNAL
//!                 Instruction::StopSignal(StopSignal::ByName("SIGTERM".into())),
//!                 // ONBUILD
//!                 Instruction::OnBuild(OnBuild {
//!                     instruction: Box::new(Instruction::Run(Run {
//!                         argv: vec!["echo".into(), "onbuild triggered".into()],
//!                         mounts: vec![],
//!                         network: None,
//!                         security: None,
//!                     })),
//!                 }),
//!                 // RUN (all mount types)
//!                 Instruction::Run(Run {
//!                     argv: vec![
//!                         "bash".into(),
//!                         "-c".into(),
//!                         "echo building && make all".into(),
//!                     ],
//!                     mounts: vec![
//!                         RunMount::Bind(RunMountBind {
//!                             target: "/mnt/bind".into(),
//!                             opts: vec![RunMountBindOpts::ReadWrite],
//!                         }),
//!                         RunMount::Cache(RunMountCache {
//!                             target: "/mnt/cache".into(),
//!                             opts: vec![
//!                                 RunMountCacheOpts::Id("build-cache".into()),
//!                                 RunMountCacheOpts::Sharing(RunSharing::Shared),
//!                                 RunMountCacheOpts::Uid(1000),
//!                                 RunMountCacheOpts::Gid(1000),
//!                                 RunMountCacheOpts::Mode("0o755".into()),
//!                             ],
//!                         }),
//!                         RunMount::Ssh(RunMountSSH {
//!                             target: None,
//!                             opts: vec![
//!                                 RunMountSSHOpts::Id("default".into()),
//!                                 RunMountSSHOpts::Required,
//!                             ],
//!                         }),
//!                         RunMount::Secret(RunMountSecret {
//!                             target: Some("/run/secrets/mysecret".into()),
//!                             opts: vec![
//!                                 RunMountSecretOpts::Id("mysecret".into()),
//!                                 RunMountSecretOpts::Required,
//!                             ],
//!                         }),
//!                         RunMount::Tmpfs(RunMountTmpfs {
//!                             target: "/mnt/tmpfs".into(),
//!                             opts: vec![RunMountTmpfsOpts::Size("65536".into())],
//!                         }),
//!                     ],
//!                     network: Some(RunNetwork::Host),
//!                     security: Some(RunSecurity::Sandbox),
//!                 }),
//!                 // RUN (alternate flags)
//!                 Instruction::Run(Run {
//!                     argv: vec!["echo".into(), "isolated step".into()],
//!                     mounts: vec![],
//!                     network: Some(RunNetwork::None),
//!                     security: Some(RunSecurity::Insecure),
//!                 }),
//!                 // CMD
//!                 Instruction::Cmd(Cmd {
//!                     argv: vec!["./app".into()],
//!                 }),
//!                 // ENTRYPOINT
//!                 Instruction::Entrypoint(Entrypoint {
//!                     argv: vec!["/app/entrypoint.sh".into()],
//!                 }),
//!             ],
//!         }],
//!     };
//!
//!     let dockerfile_built = format!("{dockerfile}");
//!     assert_eq!(
//!         dockerfile_built,
//!         r##"# Full instruction coverage test scenario
//!
//! ARG BASE_IMAGE=ubuntu:24.04
//! LABEL prefrom.label.example=true
//! FROM $BASE_IMAGE
//! MAINTAINER Example Maintainer <maintainer@example.com>
//! ARG BUILD_MODE=release
//! ENV RUST_LOG=info
//! WORKDIR /app
//! USER root
//! ADD --chown=0:0 "./archive.tar.gz" "/app/"
//! COPY --chown=0:0 "./src" "/app/src"
//! EXPOSE 8080/tcp
//! EXPOSE 9090/udp
//! LABEL app.name=example
//! VOLUME ["/var/lib/appdata"]
//! SHELL ["/bin/bash", "-c"]
//! HEALTHCHECK --interval=30s --timeout=3s --retries=3 CMD ["curl", "-f", "http://localhost:8080/health", "||", "exit", "1"]
//! STOPSIGNAL SIGTERM
//! ONBUILD RUN ["echo", "onbuild triggered"]
//! RUN --mount=type=bind,target=/mnt/bind,readwrite=true --mount=type=cache,target=/mnt/cache,id=build-cache,sharing=shared,uid=1000,gid=1000,mode=0o755 --mount=type=ssh,id=default,required=true --mount=type=secret,source=/run/secrets/mysecret,id=mysecret,required=true --mount=type=tmpfs,target=/mnt/tmpfs,size=65536 --network=host --security=sandbox ["bash", "-c", "echo building && make all"]
//! RUN --network=none --security=insecure ["echo", "isolated step"]
//! CMD ["./app"]
//! ENTRYPOINT ["/app/entrypoint.sh"]
//! "##
//!     );
//! }
//! ```

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

mod from;
mod instruction;

pub use from::From;
pub use instruction::*;

#[derive(Clone, Debug)]
pub struct Dockerfile {
    pub config: Option<DockerfileConfig>,
    pub comment: Option<String>,
    pub stages: Vec<Stage>,
}
impl Display for Dockerfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.config {
            Some(config) => writeln!(f, "{config}")?,
            None => {}
        }

        match &self.comment {
            Some(comment) => {
                for line in comment.lines() {
                    writeln!(f, "# {line}")?;
                }
                writeln!(f, "")?;
            }
            None => {}
        }

        for (e, i) in self.stages.iter().enumerate() {
            if e != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{i}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct DockerfileConfig {
    pub syntax: Option<String>,
    pub escape: Option<char>,
    pub check_skips: Option<CheckSkips>,
    pub check_error: Option<bool>,
}
impl Display for DockerfileConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.syntax {
            Some(syntax) => writeln!(f, "# syntax={syntax}")?,
            None => {}
        }
        match &self.escape {
            Some(escape) => writeln!(f, "# escape={escape}")?,
            None => {}
        }
        match &self.check_skips {
            Some(check_skips) => {
                write!(f, "# check=skip={check_skips}")?;

                match &self.check_error {
                    Some(check_error) => write!(f, ";error={check_error}\n")?,
                    None => write!(f, "\n")?,
                }
            }
            None => match &self.check_error {
                Some(check_error) => writeln!(f, "# check=error={check_error}")?,
                None => {}
            },
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum CheckSkips {
    All,
    Some(Vec<String>),
}
impl Display for CheckSkips {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Some(e) => {
                for (i, check) in e.iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{check}")?;
                }
                Ok(())
            }
        }
    }
}

/// The Stage is the unit which sits above the Instruction. It contains a vector of instructions to be placed before and after the FROM command
#[derive(Clone, Debug)]
pub struct Stage {
    pub pre_from_instructions: Vec<PreFromInstruction>,
    pub from: From,
    pub instructions: Vec<Instruction>,
}
impl Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for i in &self.pre_from_instructions {
            writeln!(f, "{i}")?;
        }
        writeln!(f, "{}", self.from)?;
        for i in &self.instructions {
            writeln!(f, "{i}")?;
        }

        Ok(())
    }
}
