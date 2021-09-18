use dxfio::AtomList;
use std::ops::Deref;

fn main() {
    env_logger::init();
    let (dxf_path, svg_path) = {
        let mut args = std::env::args();
        args.next();
        (args.next().unwrap(), args.next().unwrap())
    };
    println!("dxf_path = {}, svg_path = {}", dxf_path, svg_path);

    let drawing = dxfio::Drawing::open(&dxf_path).unwrap();

    let view_box = {
        let min = &drawing
            .headers
            .iter()
            .find(|node| node.node_type == "$LIMMIN")
            .unwrap()
            .atoms
            .deref()
            .get_point(0);
        let max = &drawing
            .headers
            .iter()
            .find(|node| node.node_type == "$LIMMAX")
            .unwrap()
            .atoms
            .deref()
            .get_point(0);
        (min[0], min[1], max[0] - min[0], max[1] - min[1])
    };
    let mut svg = svg::Document::new().set("viewBox", view_box).add(
        svg::node::element::Rectangle::new()
            .set("fill", "white")
            .set("x", view_box.0)
            .set("y", view_box.0)
            .set("width", view_box.2 - view_box.0)
            .set("height", view_box.3 - view_box.1),
    );
    for entity in &drawing.entities {
        svg = draw_entity(svg, entity, &drawing, &|p| {
            [p[0], 2.0 * view_box.1 + view_box.3 - p[1], p[2]]
        });
    }
    svg::save(svg_path, &svg).unwrap();
}

fn draw_entity(
    mut svg: svg::Document,
    entity: &dxfio::EntityNode,
    drawing: &dxfio::Drawing,
    transform: &dyn Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    match &entity.entity {
        dxfio::Entity::Insert(insert) => {
            svg = draw_insert(svg, insert, drawing, transform);
        }
        dxfio::Entity::Line(line) => {
            svg = draw_line(svg, line, transform);
        }
        dxfio::Entity::NotSupported(_, _) => {}
        _ => unimplemented!(),
    }
    svg
}

fn draw_insert(
    mut svg: svg::Document,
    insert: &dxfio::Insert,
    drawing: &dxfio::Drawing,
    transform: &dyn Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    if let Some(block) = drawing
        .blocks
        .iter()
        .find(|block| block.block_name == insert.block_name)
    {
        let (cos, sin) = {
            let theta = insert.rotation_degree * std::f64::consts::PI / 180.0;
            (theta.cos(), theta.sin())
        };
        let transform = |p: &[f64; 3]| {
            let p = [
                insert.scale_factor[0] * p[0],
                insert.scale_factor[1] * p[1],
                insert.scale_factor[2] * p[2],
            ];
            let p = [cos * p[0] - sin * p[1], sin * p[0] + cos * p[1], p[2]];
            let p = [
                insert.insertion_point[0] + p[0],
                insert.insertion_point[1] + p[1],
                insert.insertion_point[2] + p[2],
            ];
            transform(&p)
        };
        for entity in &block.entities {
            svg = draw_entity(svg, entity, drawing, &transform);
        }
    } else {
        println!("block not found: name = {}", insert.block_name);
    }
    svg
}

fn draw_line(
    svg: svg::Document,
    line: &dxfio::Line,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    line_strip(svg, &[transform(&line.p1), transform(&line.p2)], None)
}

fn line_strip(svg: svg::Document, pol: &[[f64; 3]], color: Option<&'static str>) -> svg::Document {
    if pol.len() >= 2 {
        let data = svg::node::element::path::Data::new().move_to((pol[0][0], pol[0][1]));
        let data = (1..pol.len())
            .map(|i| (pol[i][0] - pol[i - 1][0], pol[i][1] - pol[i - 1][1]))
            .fold(data, |data, v| data.line_by(v));
        let path = svg::node::element::Path::new()
            .set("fill", "none")
            .set("stroke", color.unwrap_or("black"))
            .set("stroke-width", 1)
            .set("d", data);
        svg.add(path)
    } else {
        svg
    }
}
