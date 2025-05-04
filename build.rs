use anyhow::Result;

fn main() -> Result<()> {
    // Configure vergen
    let build = vergen_gitcl::GitclBuilder::all_git()?;
    vergen_gitcl::Emitter::default().add_instructions(&build)?.emit()?;

    Ok(())
}
