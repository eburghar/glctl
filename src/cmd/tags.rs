use crate::{
	args::{self, TagsCmd},
	context::CliContext,
};

use anyhow::{Context, Result};
use gitlab::{
	api::{
		self,
		projects::protected_tags::{ProtectTag, ProtectedTags, UnprotectTag},
		Query,
	},
	types,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Tag {
	name: String,
}

pub fn cmd(context: &CliContext, args: &args::Tags) -> Result<()> {
	match &args.cmd {
		TagsCmd::Unprotect(args) => {
			let project = context.get_project(args.project.as_ref())?;
			let tag = context.get_tagexp(Some(&args.tag))?;

			let endpoint = ProtectedTags::builder()
				.project(project.path_with_namespace.to_owned())
				.build()?;
			let tags: Vec<types::ProtectedTag> = endpoint.query(&context.gitlab)?;

			if tags.iter().find(|t| &t.name == tag).is_none() {
				println!(
					"tag '{}' protection not found on project {}",
					&tag, &project.path_with_namespace
				);
			} else {
				let endpoint = UnprotectTag::builder()
					.project(project.path_with_namespace.to_owned())
					.name(tag)
					.build()?;
				api::ignore(endpoint).query(&context.gitlab)?;
				println!(
					"tag '{}' protection has been removed on project {}",
					&tag, &project.path_with_namespace
				);
			}

			if context.open {
				let _ = open::that(format!("{}/-/settings/repository", project.web_url));
			}

			Ok(())
		}

		TagsCmd::Protect(args) => {
			let project = context.get_project(args.project.as_ref())?;
			let tag = context.get_tagexp(Some(&args.tag))?;

			let endpoint = ProtectedTags::builder()
				.project(project.path_with_namespace.to_owned())
				.build()?;
			let tags: Vec<types::ProtectedTag> = endpoint.query(&context.gitlab)?;

			if tags.iter().find(|t| &t.name == tag).is_some() {
				println!(
					"tag '{}' protection already added on project {}",
					&tag, &project.path_with_namespace
				);
			} else {
				let endpoint = ProtectTag::builder()
					.project(project.path_with_namespace.to_owned())
					.name(tag)
					.build()?;
				let tag: Tag = endpoint.query(&context.gitlab).with_context(|| {
					format!(
						"Failed to protect tag '{}' on project {}",
						&tag, &project.path_with_namespace
					)
				})?;
				println!(
					"tag '{}' is protected on project {}",
					tag.name, &project.path_with_namespace
				);
			}

			if context.open {
				let _ = open::that(format!("{}/-/settings/repository", project.web_url));
			}

			Ok(())
		}
	}
}
