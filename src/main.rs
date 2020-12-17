mod request;
mod schemas;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. met ids from bigg
    // TODO: parse bigg universal
    let bigg = ["199420", "114984"];
    for id in bigg.iter() {
        // 2. request + parse
        let res = request::get_pathway(schemas::Met {
            id,
            species: "9606",
        })
        .await?;
        let path: Result<Vec<schemas::Pathway>,  serde_json::Error> = serde_json::from_str(res.as_str());
        match path {
            // TODO: async write to file WIP
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
