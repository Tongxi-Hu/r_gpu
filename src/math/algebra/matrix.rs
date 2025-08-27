use std::ops::{Index, IndexMut, Mul};

use crate::math::algebra::{common::deg_to_rad, point::Point, vector::Vector};

use super::common::{Determinant, Dimension4, FuzzyEq};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix<const D: usize> {
    pub data: [[f32; D]; D],
}

unsafe impl<const D: usize> bytemuck::Zeroable for Matrix<D> {}

unsafe impl<const D: usize> bytemuck::Pod for Matrix<D> {}

impl<const D: usize> Matrix<D> {
    pub fn new() -> Self {
        Self {
            data: [[0.0; D]; D],
        }
    }

    pub fn diagonal(value: f32) -> Matrix<D> {
        let mut matrix = Matrix::new();
        for i in 0..D {
            matrix[i][i] = value;
        }
        matrix
    }

    pub fn identity() -> Self {
        Matrix::diagonal(1.0)
    }

    pub fn transpose(&self) -> Self {
        let mut matrix = Matrix::new();
        for row in 0..D {
            for col in 0..D {
                matrix[col][row] = self[row][col];
            }
        }
        matrix
    }

    pub fn get_raw(&self) -> [[f32; D]; D] {
        return self.data;
    }
}

impl<const D: usize> Default for Matrix<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [f32; D];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const D: usize> FuzzyEq for Matrix<D> {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        for row in 0..D {
            for col in 0..D {
                if !self[row][col].fuzzy_eq(&other[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl<const D: usize> PartialEq for Matrix<D> {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl<const D: usize> Mul for Matrix<D> {
    type Output = Matrix<D>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res: Matrix<D> = Matrix::new();
        for r in 0..D {
            for c in 0..D {
                for i in 0..D {
                    res[r][c] = res[r][c] + self[r][i] * rhs[i][c];
                }
            }
        }
        res
    }
}

impl Determinant for Matrix<2> {
    fn det(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Determinant for Matrix<3> {
    fn det(&self) -> f32 {
        self[0][0] * self[1][1] * self[2][2]
            + self[0][1] * self[1][2] * self[2][1]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][2] * self[1][1] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            - self[0][0] * self[1][2] * self[2][1]
    }
}

impl Determinant for Matrix<4> {
    fn det(&self) -> f32 {
        let mut det: f32 = 0.0;
        for col in 0..4 {
            det = det + self.cofactor(0, col) * self[0][col]
        }
        det
    }
}

impl Matrix<4> {
    fn sub(&self, row: usize, column: usize) -> Matrix<3> {
        let mut matrix: Matrix<3> = Matrix::new();
        let mut source_row: usize = 0;
        let mut source_column: usize = 0;
        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 3 {
            if source_row == row {
                // Skip row to be removed
                source_row += 1;
            }
            while target_column < 3 {
                if source_column == column {
                    // Skip column to be removed
                    source_column += 1;
                }
                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }
            source_row += 1;
            source_column = 0;
            target_row += 1;
            target_column = 0;
        }

        matrix
    }

    fn cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 == 0 {
            self.sub(row, col).det()
        } else {
            -self.sub(row, col).det()
        }
    }

    pub fn is_invertible(&self) -> bool {
        !self.det().fuzzy_eq(&0.0)
    }

    pub fn inverse(&self) -> Result<Self, String> {
        if self.is_invertible() {
            let det = self.det();
            let mut inverse: Matrix<4> = Matrix::new();
            for row in 0..4 {
                for col in 0..4 {
                    let cofactor = self.cofactor(row, col);
                    inverse[col][row] = cofactor / det;
                }
            }
            Ok(inverse)
        } else {
            Err(String::from("not invertible"))
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Matrix::<4> {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Matrix::<4> {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_x(deg: f32) -> Self {
        let r = deg_to_rad(deg);
        Matrix::<4> {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, r.cos(), -r.sin(), 0.0],
                [0.0, r.sin(), r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_y(deg: f32) -> Self {
        let r = deg_to_rad(deg);
        Matrix::<4> {
            data: [
                [r.cos(), 0.0, r.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-r.sin(), 0.0, r.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_z(deg: f32) -> Self {
        let r = deg_to_rad(deg);
        Matrix::<4> {
            data: [
                [r.cos(), -r.sin(), 0.0, 0.0],
                [r.sin(), r.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // view:(with,height,near,far)
    pub fn perspective(view: Point, eye: Point, _eye_direction: Vector) -> Self {
        let view = view.get_raw();
        // move eye to the origin
        let translation = Self::translation(-eye.get_x(), -eye.get_y(), -eye.get_z());
        //TODO: look to negative z direction
        let rotation = Matrix::identity();

        let perspective_projection = Matrix {
            data: [
                [2.0 * (-view[2]) / view[0], 0.0, 0.0, 0.0], // x
                [0.0, 2.0 * (-view[2]) / view[1], 0.0, 0.0], // y
                [
                    0.0,
                    0.0,
                    (-view[3]) / (view[3] - view[2]),
                    view[2] * view[3] / (view[3] - view[2]),
                ], // z
                [0.0, 0.0, -1.0, 0.0],
            ],
        };
        return perspective_projection * rotation * translation;
    }
}

impl<T: Dimension4<Value = f32>> Mul<T> for Matrix<4> {
    type Output = T;
    fn mul(self, other: T) -> Self::Output {
        T::new(
            self[0][0] * other.get_x()
                + self[0][1] * other.get_y()
                + self[0][2] * other.get_z()
                + self[0][3] * other.get_w(),
            self[1][0] * other.get_x()
                + self[1][1] * other.get_y()
                + self[1][2] * other.get_z()
                + self[1][3] * other.get_w(),
            self[2][0] * other.get_x()
                + self[2][1] * other.get_y()
                + self[2][2] * other.get_z()
                + self[2][3] * other.get_w(),
            self[3][0] * other.get_x()
                + self[3][1] * other.get_y()
                + self[3][2] * other.get_z()
                + self[3][3] * other.get_w(),
        )
    }
}

#[test]
fn test_matrix() {
    use crate::math::algebra::vector::Vector;

    let vector = Matrix::<4>::rotate_z(std::f32::consts::PI / 2.0) * Vector::vector(1.0, 0.0, 0.0);
    assert!(Vector::vector(0.0, 1.0, 0.0).fuzzy_eq(&vector))
}
