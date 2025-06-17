use git2::Repository;
use serde_json::json;
use std::{fs, path::Path};

/// Exporte jusqu'à 100 commits en JSON.
/// `repo_path` = chemin du dépôt à analyser.
/// `out_path`  = fichier de sortie.
pub fn export_json<P: AsRef<Path>>(repo_path: P, out_path: P) -> anyhow::Result<()> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;                 // parcours depuis HEAD
    let mut commits = Vec::new();

    for oid_result in revwalk.take(100) { // 100 commits pour le POC
        let oid   = oid_result?;
        let c     = repo.find_commit(oid)?;
        commits.push(json!({
            "hash":    c.id().to_string(),
            "author":  c.author().name().unwrap_or(""),
            "date":    c.time().seconds(),   // timestamp Unix
            "message": c.summary().unwrap_or("")
        }));
    }

    fs::write(out_path, serde_json::to_vec_pretty(&commits)?)?;
    Ok(())
}

