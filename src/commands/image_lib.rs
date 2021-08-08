#[allow(dead_code)]
pub async fn fetch_url(
    url: String,
    file_name: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut file = std::fs::File::create(file_name)?;
    let mut content = std::io::Cursor::new(reqwest::get(url).await?.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
