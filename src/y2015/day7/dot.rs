use std::fs::File;
use std::io::Write;

use crate::y2015::day7::{Instruction, Operation};

pub fn generate_dot_file(instructions: Vec<Instruction>) {
    let all_nodes: Vec<String> = instructions
        .iter()
        .flat_map(|i| {
            let mut v: Vec<String> = match i.operation.clone() {
                Operation::And(a, b) => [a, b].to_vec(),
                Operation::Or(a, b) => [a, b].to_vec(),
                Operation::Rshift(a, _) => [a].to_vec(),
                Operation::Lshift(a, _) => [a].to_vec(),
                Operation::Not(a) => [a].to_vec(),
                Operation::ValueInt(a) => [a.to_string()].to_vec(),
                Operation::WireValue(a) => [a].to_vec(),
            };

            v.push(i.target_wire.clone());
            v
        })
        .collect();

    let edges: Vec<String> = instructions
        .iter()
        .flat_map(|i| {
            let v: Vec<String> = match i.operation.clone() {
                Operation::And(a, b) => [
                    format!("{} -> {};", a, i.target_wire),
                    format!("{} -> {};", b, i.target_wire),
                ]
                .to_vec(),
                Operation::Or(a, b) => [
                    format!("{} -> {};", a, i.target_wire),
                    format!("{} -> {};", b, i.target_wire),
                ]
                .to_vec(),
                Operation::Rshift(a, _) => [format!("{} -> {};", a, i.target_wire)].to_vec(),
                Operation::Lshift(a, _) => [format!("{} -> {};", a, i.target_wire)].to_vec(),
                Operation::Not(a) => [format!("{} -> {};", a, i.target_wire)].to_vec(),
                Operation::ValueInt(a) => [format!("{} -> {};", a, i.target_wire)].to_vec(),
                Operation::WireValue(a) => [format!("{} -> {};", a, i.target_wire)].to_vec(),
            };
            v
        })
        .collect();

    let node_defs: Vec<String> = all_nodes
        .iter()
        .map(|node| {
            let (shape, color) = ("box", "lightblue");
            format!("        {node} [shape={shape}, color={color}, style=filled];")
        })
        .collect();

    let dot_content = format!(
        r#"digraph Day11 {{
            // Graph settings
            rankdir=TB;
            splines=ortho;
            nodesep=0.5;
            ranksep=0.8;
            
            // Node defaults
            node [fontname="Helvetica", fontsize=12];
            edge [color=gray50];
            
            // Nodes
            {}

            // Edges
            {}
        }}
        "#,
        node_defs.join("\n"),
        edges.join("\n")
    );

    let output_path = "src/y2015/day7/dot.dot";
    let mut file = File::create(output_path).unwrap();
    file.write_all(dot_content.as_bytes()).unwrap();
    println!("Generated DOT file: {}", output_path);
}
