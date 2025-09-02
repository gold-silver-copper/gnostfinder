use crate::*;

pub fn astar_sentence(
    graph: &ThingGraph,
    triples: Vec<(NodeIndex, GameEdge, NodeIndex)>,
    perspective_id: NodeIndex,
) -> String {
    let mut description = String::new();

    let mut mentioned_nodes = HashSet::new();

    let prev_target = NodeIndex::from(u32::MAX);

    let mut first_iter = true;

    for (source, edge_type, target) in triples {
        let person = if source == perspective_id {
            Person::Second
        } else {
            Person::Third
        };

        let source_name = if person == Person::Second {
            "You are".into()
        } else {
            format!("{} is", graph[source].name())
        };
        let determiner = if mentioned_nodes.contains(&target) {
            "the"
        } else {
            "a"
        };

        if first_iter {
            description.push_str(&format!("{source_name}"));
            first_iter = false;
        }

        let target_name = graph[target].name(); // use display_name() if needed

        let phrase = edge_type.get_word();

        description.push_str(&format!(" {phrase} {determiner} {target_name}"));
        mentioned_nodes.insert(source);
        mentioned_nodes.insert(target);
    }
    description
}
