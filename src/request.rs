use super::schemas::Met;
use reqwest::Client;

pub async fn get_pathway(met: Met<'_>) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let request = client
        .get(
            format!(
                "https://reactome.org/ContentService/data/pathways/low/entity/R-HSA-{}?species={}",
                met.id, met.species
            )
            .as_str(),
        )
        .header("Accept", "application/json");
    let res = request.send().await?;
    let body = res.bytes().await?;
    Ok(String::from_utf8(body.to_vec())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn request_valid_reactome_met() {
        let res = get_pathway(Met {
            id: "199420",
            species: "9606",
        })
        .await
        .unwrap();

        res.find("Synthesis of PIPs at the plasma membrane")
            .unwrap();
    }
}
