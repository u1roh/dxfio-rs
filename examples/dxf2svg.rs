mod geom2d;
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

    let drawing = dxfio::Document::open(&dxf_path).unwrap();

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
            .set("y", view_box.1)
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
    svg: svg::Document,
    entity: &dxfio::EntityNode,
    doc: &dxfio::Document,
    transform: &dyn Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    match &entity.entity {
        dxfio::Entity::Insert(insert) => draw_insert(svg, insert, doc, transform),
        dxfio::Entity::Dimension(dim) => draw_dimension(svg, dim, doc, transform),
        dxfio::Entity::Text(text) => draw_text(svg, text, transform),
        dxfio::Entity::MText(mtext) => draw_mtext(svg, mtext, transform),
        dxfio::Entity::Point(_) => {
            log::warn!("draw_entity() for Point entity: unimplemented");
            svg
        }
        dxfio::Entity::Line(line) => draw_line(svg, line, transform),
        dxfio::Entity::Circle(cir) => draw_circle(svg, cir, transform),
        dxfio::Entity::Arc(_) => {
            log::warn!("draw_entity() for Arc entity: unimplemented");
            svg
        }
        dxfio::Entity::LwPolyline(pol) => draw_lw_polyline(svg, pol, transform),
        dxfio::Entity::NotSupported(entity_type, _) => {
            log::warn!("not supported entity type: {}", entity_type);
            svg
        }
    }
}

fn draw_insert(
    mut svg: svg::Document,
    insert: &dxfio::Insert,
    doc: &dxfio::Document,
    transform: &dyn Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    if let Some(block) = doc
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
            svg = draw_entity(svg, entity, doc, &transform);
        }
    } else {
        println!("block not found: name = {}", insert.block_name);
    }
    svg
}

fn points_to_pathdata(pol: &[[f64; 3]]) -> svg::node::element::path::Data {
    let data = svg::node::element::path::Data::new().move_to((pol[0][0], pol[0][1]));
    pol[1..]
        .iter()
        .fold(data, |data, p| data.line_to((p[0], p[1])))
}

fn create_path(data: svg::node::element::path::Data) -> svg::node::element::Path {
    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data)
}

fn line_strip(svg: svg::Document, pol: &[[f64; 3]], color: Option<&'static str>) -> svg::Document {
    if pol.len() >= 2 {
        svg.add(create_path(points_to_pathdata(pol)).set("stroke", color.unwrap_or("black")))
    } else {
        svg
    }
}

fn draw_dimension(
    mut svg: svg::Document,
    dim: &dxfio::Dimension,
    doc: &dxfio::Document,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    if let Some(block) = doc
        .blocks
        .iter()
        .find(|block| block.block_name == dim.block_name)
    {
        for entity in &block.entities {
            svg = draw_entity(svg, entity, doc, &transform);
        }
        svg
    } else {
        let p1 = &dim.definition_point;
        let p2 = dim.definition_point2.as_ref().unwrap_or(&[0.0, 0.0, 0.0]);
        let p3 = dim.definition_point3.as_ref().unwrap_or(&[0.0, 0.0, 0.0]);
        let p4 = {
            let theta = dim.rotation_angle.unwrap_or(0.0) * std::f64::consts::PI / 180.0;
            let line1 = geom2d::Line {
                p: p1.into(),
                v: geom2d::UnitVec::of_angle(theta),
            };
            let line2 = geom2d::Line {
                p: p2.into(),
                v: (geom2d::Pos::from(p1) - geom2d::Pos::from(p3))
                    .normalize()
                    .unwrap(),
            };
            line1.intersection_pos(&line2).unwrap()
        };
        line_strip(
            svg,
            &[
                transform(p2),
                transform(&p4.into()),
                transform(p1),
                transform(p3),
            ],
            Some("blue"),
        )
    }
}

fn draw_text(
    svg: svg::Document,
    src: &dxfio::Text,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    let scale = (transform(&[1.0, 0.0, 0.0])[0] - transform(&[0.0, 0.0, 0.0])[0]).abs();
    // println!("draw_text(): scale = {}", scale);
    let p = transform(&src.point1);
    let text = svg::node::element::Text::new()
        .set("x", p[0])
        .set("y", p[1])
        .set("font-size", scale * src.height)
        .add(svg::node::Text::new(src.text.clone()));
    let text = if let Some(deg) = src.rotation_degree {
        println!("draw_text(): rotation_degree = {}", deg);
        text.set("transform", format!("rotate({} {} {})", -deg, p[0], p[1]))
    } else {
        text
    };
    svg.add(text)
}

fn draw_mtext(
    mut svg: svg::Document,
    mtext: &dxfio::MText,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    let scale = (transform(&[1.0, 0.0, 0.0])[0] - transform(&[0.0, 0.0, 0.0])[0]).abs();
    // println!("draw_mtext(): scale = {}", scale);
    log::info!("mtext.x_axis = {:?}", mtext.x_axis);
    log::info!("mtext.rotation_radian = {:?}", mtext.rotation_radian);
    log::info!("mtext.rectangle_width = {}", mtext.rectangle_width);
    log::info!("mtext.character_width = {}", mtext.character_width);
    svg = {
        let p = transform(&mtext.point);
        let text = mtext
            .text
            .nodes
            .iter()
            .filter_map(|node| match node {
                dxfio::MTextNode::Text(s) => Some(s as _),
                _ => None,
            })
            .collect::<Vec<&str>>()
            .join("");
        let text = svg::node::element::Text::new()
            .set("x", p[0])
            .set("y", p[1])
            .set("font-size", scale * mtext.height)
            .add(svg::node::Text::new(text));
        let text = if let Some([x, y, _]) = mtext.x_axis {
            let deg = y.atan2(x) * 180.0 / std::f64::consts::PI;
            println!("draw_mtext(): deg = {}", deg);
            text.set("transform", format!("rotate({} {} {})", -deg, p[0], p[1]))
        } else {
            text
        };
        svg.add(text)
    };
    svg = {
        let rect = {
            let u: geom2d::Vec = mtext.x_axis.unwrap_or([1.0, 0.0, 0.0]).into();
            let v = geom2d::Vec { x: -u.y, y: u.x };
            let p0: geom2d::Pos = mtext.point.into();
            let p1 = p0 + mtext.rectangle_width * u;
            let p2 = p1 + mtext.height * v;
            let p3 = p0 + mtext.height * v;
            [
                transform(&p0.into()),
                transform(&p1.into()),
                transform(&p2.into()),
                transform(&p3.into()),
                transform(&p0.into()),
            ]
        };
        line_strip(svg, &rect, Some("blue"))
    };
    svg
}

fn draw_line(
    svg: svg::Document,
    line: &dxfio::Line,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    svg.add(create_path(points_to_pathdata(&[
        transform(&line.p1),
        transform(&line.p2),
    ])))
}

fn draw_lw_polyline(
    svg: svg::Document,
    pol: &dxfio::LwPolyline,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    let points = pol
        .vertices
        .iter()
        .map(|v| transform(&[v.coord[0], v.coord[1], 0.0]))
        .collect::<Vec<_>>();
    let mut data = points_to_pathdata(&points);
    if pol.is_closed {
        data = data.close();
    }
    svg.add(create_path(data))
}

fn draw_circle(
    svg: svg::Document,
    cir: &dxfio::Circle,
    transform: impl Fn(&[f64; 3]) -> [f64; 3],
) -> svg::Document {
    let center = transform(&cir.center);
    let element = svg::node::element::Circle::new()
        .set("cx", center[0])
        .set("cy", center[1])
        .set("r", cir.radius)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);
    svg.add(element)
}
