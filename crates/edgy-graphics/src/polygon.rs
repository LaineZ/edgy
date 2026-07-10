use crate::{Color, Fixed, draw::hline, framebuffer::FrameBuffer, geometry::Point};

struct Edge {
    y_max: i32,
    x: Fixed,
    dx: Fixed,
}

#[derive(Default)]
pub struct PolygonData<const N: usize> {
    edges: heapless::Vec<(i32, Edge), N>,
    active_edges: heapless::Vec<Edge, N>,
    pub points: heapless::Vec<Point, N>,
}

impl<const N: usize> PolygonData<N> {
    pub fn new() -> Self {
        Self::default()
    }
}

fn make_edge(a: Point, b: Point) -> Option<(i32, Edge)> {
    let (top, bottom) = if a.y < b.y { (a, b) } else { (b, a) };

    if top.y == bottom.y {
        return None;
    }

    let dy = bottom.y - top.y;

    Some((
        top.y,
        Edge {
            y_max: bottom.y,
            x: Fixed::from_num(top.x),
            dx: Fixed::from_num(bottom.x - top.x) / Fixed::from_num(dy),
        },
    ))
}

pub fn fill_polygon<const N: usize>(fb: &mut FrameBuffer, store: &mut PolygonData<N>, color: Color) {
    if store.points.len() < 3 {
        return;
    }

    store.edges.clear();
    store.active_edges.clear();

    let min_y = store.points.iter().map(|p| p.y).min().unwrap();
    let max_y = store.points.iter().map(|p| p.y).max().unwrap();

    for i in 0..store.points.len() {
        if let Some(edge) = make_edge(store.points[i], store.points[(i + 1) % store.points.len()]) {
            let _ = store.edges.push(edge);
        }
    }

    for y in min_y..max_y {
        // add new edges
        for (start, edge) in store.edges.iter() {
            if *start == y {
                let _ = store.active_edges.push(Edge {
                    y_max: edge.y_max,
                    x: edge.x,
                    dx: edge.dx,
                });
            }
        }

        // delete endpoints
        store.active_edges.retain(|e| e.y_max > y);

        // sort by x
        store.active_edges.sort_by_key(|a| a.x);

        // draw
        for pair in store.active_edges.chunks_exact(2) {
            let x0 = pair[0].x.floor().to_num::<i32>();
            let x1 = pair[1].x.floor().to_num::<i32>();

            hline(fb, x0, x1, y, color);
        }

        // next scanline
        for edge in &mut store.active_edges {
            edge.x += edge.dx;
        }
    }
}
