use neo4rs::*;

mod container;

#[tokio::test]
async fn relationships() {
    let neo4j = container::Neo4jContainer::new().await;
    let graph = neo4j.graph();

    let mut result = graph.execute(
        query("CREATE (p:Person { name: 'Oliver Stone' })-[r:WORKS_AT {as: 'Engineer'}]->(neo) RETURN r")
    ).await.unwrap();

    let row = result.next().await.unwrap().unwrap();
    let relation: Relation = row.get("r").unwrap();
    assert!(relation.id() > -1);
    assert!(relation.start_node_id() > -1);
    assert!(relation.end_node_id() > -1);
    assert_eq!(relation.typ(), "WORKS_AT");
    assert_eq!(relation.get::<String>("as").unwrap(), "Engineer");
}
