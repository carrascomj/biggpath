mod request;
mod schemas;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for id in ["199420", "114984"].iter() {
        let res = request::get_pathway(schemas::Met {
            id,
            species: "9606",
        })
        .await?;
        let path: Result<Vec<schemas::Pathway>,  serde_json::Error> = serde_json::from_str(res.as_str());
        match path {
            Ok(p) => println!("Met id: {}\tPath id: {}", id, p[0].db_id),
            _ => println!("Requested id {} not found 404", id)
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_a_pathway_from_reactome_met(){
        let res = request::get_pathway(schemas::Met {
            id: "199420",
            species: "9606",
        })
        .await.unwrap();
        
        let path: Vec<schemas::Pathway> = serde_json::from_str(res.as_str()).unwrap();
        assert_eq!(path[0].db_id, 1660499i32);
    }

}
