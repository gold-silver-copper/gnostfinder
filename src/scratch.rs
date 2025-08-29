  description.push_str(&format!("                        "));

  if let Some(in_id) = in_id {
      for edge in self.edges_directed(in_id, Direction::Incoming) {
          let source = edge.source();
          let target = edge.target();
          let target_name = self[target].name(); // use display_name() if needed
          let source_name = self[source].name(); // use display_name() if needed
          let edge_type = edge.weight();
          let phrase = edge_type.describe_to(&target_name);

          description.push_str(&format!("{source_name} {}, ", phrase));
      }
  }
