use crate::fruchterman::fruchterman_reingold;
use crate::graph::Graph;

pub fn apply_layout(g: &mut Graph) {
    g.set_position(fruchterman_reingold(g, 1000, 1000.0, 1000.0));
}
