use crate::piece::PieceKind;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PieceKindSet<V> {
    data: [V; 7],
}

impl<V> PieceKindSet<V>
where
    V: Copy,
{
    pub fn get(&self, kind: &PieceKind) -> V {
        self.data[*kind as usize]
    }
}

impl<V> PieceKindSet<V>
where
    V: Copy,
{
    pub fn new_with_value(value: V) -> PieceKindSet<V> {
        PieceKindSet { data: [value; 7] }
    }
}
