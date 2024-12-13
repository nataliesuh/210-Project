use plotters::prelude::*;
use petgraph::graph::Graph;
use petgraph::visit::EdgeRef;
use std::error::Error;

pub fn draw_graph(
    graph: &Graph<String, f64, petgraph::Undirected>,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Team Interaction Graph", ("sans-serif", 30))
        .margin(10)
        .build_cartesian_2d(-1.5..1.5, -1.5..1.5)?;

    let mut positions = std::collections::HashMap::new();
    let nodes = graph.node_indices().collect::<Vec<_>>();
    let node_count = nodes.len();

    for (i, &node) in nodes.iter().enumerate() {
        let angle = (i as f64 / node_count as f64) * 2.0 * std::f64::consts::PI;
        positions.insert(node, (angle.cos(), angle.sin()));
    }

    for edge in graph.edge_references() {
        let (source, target) = (edge.source(), edge.target());
        if let (Some(&(x1, y1)), Some(&(x2, y2))) = (positions.get(&source), positions.get(&target)) {
            chart.draw_series(LineSeries::new(vec![(x1, y1), (x2, y2)], &BLUE))?;
        }
    }

    for (&node, &(x, y)) in &positions {
        chart.draw_series(PointSeries::of_element(
            vec![(x, y)],
            5,
            &RED,
            &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
        ))?;

        let label_offset = 0.2;
        let angle = y.atan2(x);
        let (label_x, label_y) = (x + label_offset * angle.cos(), y + label_offset * angle.sin());

        chart.draw_series(std::iter::once(Text::new(
            graph[node].clone(),
            (label_x, label_y),
            ("sans-serif", 15).into_font(),
        )))?;
    }

    root.present()?;
    Ok(())
}
