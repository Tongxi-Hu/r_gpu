#[derive(Clone, Copy, Debug)]
pub struct GridPoint<const D: usize> {
    location: [i64; D],
}
impl<const D: usize> GridPoint<D> {
    pub fn new(location: [i64; D]) -> Self {
        Self { location }
    }

    pub fn origin() -> Self {
        Self::new([0; D])
    }

    pub fn location(&self) -> [i64; D] {
        self.location
    }

    /// points adjacent to self, length : 2*D
    pub fn adjacent(&self) -> Vec<GridPoint<D>> {
        let mut adjacent: Vec<GridPoint<D>> = vec![];
        // length : 2*D
        for i in 0..D {
            let mut location = self.location;
            location[i] = location[i] - 1;
            adjacent.push(GridPoint::<D>::new(location));
            location[i] = location[i] + 2;
            adjacent.push(GridPoint::<D>::new(location))
        }
        adjacent
    }

    pub fn translate(&self, translation: [i64; D]) -> Self {
        let mut location = self.location;
        for i in 0..D {
            location[i] = location[i] + translation[i];
        }
        Self::new(location)
    }

    /// length: 3^D - 1
    pub fn neighbor(&self) -> Vec<GridPoint<D>> {
        let mut translations: Vec<[i64; D]> = vec![];
        let mut cur_move: Vec<i64> = vec![];

        fn dfs<const D: usize>(translations: &mut Vec<[i64; D]>, cur_move: &mut Vec<i64>) {
            if cur_move.len() == D {
                translations.push(cur_move.clone().try_into().unwrap_or([0; D]));
            } else {
                for step in [-1, 0, 1] {
                    cur_move.push(step);
                    dfs(translations, cur_move);
                    cur_move.pop();
                }
            }
        }

        dfs(&mut translations, &mut cur_move);
        translations
            .into_iter()
            .filter(|translate| translate.iter().any(|&step| step != 0))
            .map(|translation| self.translate(translation))
            .collect()
    }

    pub fn move_i(&self, i: usize, delta_i: i64) -> Option<Self> {
        let mut location = self.location;
        match location.get_mut(i) {
            None => None,
            Some(v) => {
                *v = *v + delta_i;
                Some(Self { location })
            }
        }
    }
}

impl<const D: usize> PartialEq for GridPoint<D> {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

#[test]
fn test_neighbor() {
    let grid = GridPoint::<4>::new([0, 1, 2, 3]);
    assert_eq!(grid.neighbor().len(), 3_usize.pow(4) - 1);
}
