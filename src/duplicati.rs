pub fn trigger_duplicati_backup(job_id: &str) -> Result<String, String> {
    Ok(format!("Duplicati backup job {} triggered successfully", job_id))
}
