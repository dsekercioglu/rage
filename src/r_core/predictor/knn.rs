
//This can later be made into a Kd-tree
#[derive(Debug, Clone)]
pub struct LinearSearcher<T: Copy> {
    data: Vec<(Vec<packed_simd::f32x4>, T)>,
    cap: usize,
}

impl<T: Copy> LinearSearcher<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            data: vec![],
            cap,
        }
    }

    pub fn add(&mut self, data_point: &[f32], value: T) {
        let search_pack = data_point
            .chunks(4)
            .map(|chunk| packed_simd::f32x4::from_slice_unaligned(chunk))
            .collect::<Vec<packed_simd::f32x4>>();
        if self.data.len() > self.cap {
            self.data.remove(0);
        }
        self.data.push((search_pack, value));
    }

    pub fn search(&mut self, search_point: &[f32]) -> Option<T> {
        let search_pack = search_point
            .chunks(4)
            .map(|chunk| packed_simd::f32x4::from_slice_unaligned(chunk))
            .collect::<Vec<packed_simd::f32x4>>();

        let mut min_distance = f32::INFINITY;
        let mut closest_index = None;
        for (index, (data_point, _)) in self.data.iter().enumerate() {
            let dist = data_point
                .iter()
                .zip(search_pack.iter())
                .map(|(d, s)| {
                    let diff = *d - *s;
                    diff * diff
                })
                .sum::<packed_simd::f32x4>()
                .sum();
            if dist < min_distance {
                min_distance = dist;
                closest_index = Some(index);
            }
        }
        if let Some(index) = closest_index {
            Some(self.data[index].1)
        } else {
            None
        }
    }
}