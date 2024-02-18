use derive_more::{Deref, From, Index, IndexMut, IntoIterator};
use glam::Affine3A;

use super::{Shape, Vertex};
/// ComplexShape that is composed of a constant size of primitives
#[derive(Debug, Clone, Copy, PartialEq, From, Index, IndexMut, IntoIterator, Deref)]
pub struct SizedComplexShape<T: Shape, const D: usize> {
    #[index]
    #[index_mut]
    #[into_iterator(owned, ref, ref_mut)]
    #[deref]
    pub primitives: [T; D],
    #[from(ignore)]
    pub model_transform: Option<Affine3A>,
}

impl<T: Shape, const D: usize> From<[T; D]> for SizedComplexShape<T, D> {
    fn from(primitives: [T; D]) -> Self {
        SizedComplexShape {
            primitives: primitives,
            model_transform: None,
        }
    }
}
impl<T: Shape, const D: usize> SizedComplexShape<T, D> {
    pub fn new(primitives: [T; D]) -> Self {
        SizedComplexShape {
            primitives: primitives,
            model_transform: None,
        }
    }
    pub fn with_opt_transform(primitives: [T; D], model_transform: Option<Affine3A>) -> Self {
        SizedComplexShape {
            primitives: primitives,
            model_transform,
        }
    }
    pub fn with_transform(primitives: [T; D], model_transform: Affine3A) -> Self {
        SizedComplexShape {
            primitives: primitives,
            model_transform: Some(model_transform),
        }
    }
    pub fn set_transform(&mut self, transform: Affine3A) {
        self.model_transform = Some(transform);
    }
}

impl<T: Shape, const D: usize> Shape for SizedComplexShape<T, D> {
    fn vertices(&self) -> Vec<Vertex> {
        complex_shape_utils::vertices(&self.primitives, self.model_transform.as_ref())
    }
    fn number_of_verticies(&self) -> usize {
        complex_shape_utils::number_of_verticies(&self.primitives)
    }
    fn push_verticies(&self, verticies: &mut Vec<Vertex>) {
        complex_shape_utils::push_verticies(
            &self.primitives,
            verticies,
            self.model_transform.as_ref(),
        );
    }
    fn indices(&self) -> Vec<u32> {
        complex_shape_utils::indices(&self.primitives)
    }
    fn push_indices(&self, indices: &mut Vec<u32>) {
        complex_shape_utils::push_indices(&self.primitives, indices);
    }
    fn number_of_indices(&self) -> usize {
        complex_shape_utils::number_of_indices(&self.primitives)
    }
    fn verticies_grouped(&self) -> Vec<[Vertex; 3]> {
        complex_shape_utils::verticies_grouped(&self.primitives, self.model_transform.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, From, Index, IndexMut, IntoIterator, Deref)]
pub struct ComplexShape<T: Shape> {
    #[index]
    #[index_mut]
    #[into_iterator(owned, ref, ref_mut)]
    #[deref]
    primitives: Vec<T>,
    #[from(ignore)]
    model_transform: Option<Affine3A>,
}

impl<T: Shape> ComplexShape<T> {
    pub fn new(primitives: Vec<T>) -> Self {
        ComplexShape {
            primitives,
            model_transform: None,
        }
    }
    pub fn with_transform(primitives: Vec<T>, model_transform: Affine3A) -> Self {
        ComplexShape {
            primitives,
            model_transform: Some(model_transform),
        }
    }
}
impl<T: Shape> Shape for ComplexShape<T> {
    fn vertices(&self) -> Vec<Vertex> {
        complex_shape_utils::vertices(&self.primitives, self.model_transform.as_ref())
    }
    fn number_of_verticies(&self) -> usize {
        complex_shape_utils::number_of_verticies(&self.primitives)
    }
    fn push_verticies(&self, verticies: &mut Vec<Vertex>) {
        complex_shape_utils::push_verticies(
            &self.primitives,
            verticies,
            self.model_transform.as_ref(),
        );
    }
    fn indices(&self) -> Vec<u32> {
        complex_shape_utils::indices(&self.primitives)
    }
    fn push_indices(&self, indices: &mut Vec<u32>) {
        complex_shape_utils::push_indices(&self.primitives, indices);
    }
    fn number_of_indices(&self) -> usize {
        complex_shape_utils::number_of_indices(&self.primitives)
    }
    fn verticies_grouped(&self) -> Vec<[Vertex; 3]> {
        complex_shape_utils::verticies_grouped(&self.primitives, self.model_transform.as_ref())
    }
}

/// Both unsized and sized complex shapes use the same implementation
/// This could change in the future. If further optimizations are found for the sized version
mod complex_shape_utils {
    use glam::Affine3A;

    use crate::engine::shapes::{Shape, Vertex};
    #[inline(always)]
    pub fn verticies_grouped<T: Shape>(
        primitives: &[T],
        model_transform: Option<&Affine3A>,
    ) -> Vec<[Vertex; 3]> {
        if let Some(transform) = model_transform {
            primitives
                .iter()
                .flat_map(|quad| quad.verticies_grouped())
                .map(|v| {
                    [
                        v[0].transform(*transform),
                        v[1].transform(*transform),
                        v[2].transform(*transform),
                    ]
                })
                .collect()
        } else {
            primitives
                .iter()
                .flat_map(|quad| quad.verticies_grouped())
                .map(|v| [v[0], v[1], v[2]])
                .collect()
        }
    }
    #[inline(always)]
    pub fn vertices<T: Shape>(primitives: &[T], model_transform: Option<&Affine3A>) -> Vec<Vertex> {
        let mut verticies = Vec::with_capacity(primitives.len() * 4);
        push_verticies(primitives, &mut verticies, model_transform);
        verticies
    }
    #[inline(always)]
    pub fn number_of_verticies<T: Shape>(primitives: &[T]) -> usize {
        primitives
            .iter()
            .map(|prim| prim.number_of_verticies())
            .sum()
    }
    #[inline(always)]
    pub fn push_verticies<T: Shape>(
        primitives: &[T],
        verticies: &mut Vec<Vertex>,
        model_transform: Option<&Affine3A>,
    ) {
        if let Some(transform) = model_transform {
            primitives
                .iter()
                .flat_map(|quad| quad.vertices())
                .map(|v| v.transform(*transform))
                .for_each(|v| verticies.push(v));
        } else {
            primitives
                .iter()
                .flat_map(|quad| quad.vertices())
                .for_each(|v| verticies.push(v));
        }
    }
    #[inline(always)]
    pub fn indices<T: Shape>(primitives: &[T]) -> Vec<u32> {
        let mut indices = Vec::with_capacity(primitives.len() * 6 * 6);
        push_indices(primitives, &mut indices);
        indices
    }
    #[inline(always)]
    pub fn push_indices<T: Shape>(primitives: &[T], indices: &mut Vec<u32>) {
        let mut offset = indices.len() as u32;
        for quad in primitives {
            let quad_indices = quad.indices();
            indices.extend(quad_indices.iter().map(|index| index + offset));
            offset += quad.vertices().len() as u32;
        }
    }
    #[inline(always)]
    pub fn number_of_indices<T: Shape>(primitives: &[T]) -> usize {
        primitives.iter().map(|prim| prim.number_of_indices()).sum()
    }
}
