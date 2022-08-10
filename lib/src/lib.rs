use nohash_hasher::IntMap;

pub trait Roll {
    fn roll(&mut self) -> usize;
}

impl<R> Roll for Vec<R>
where
    R: Roll,
{
    fn roll(&mut self) -> usize {
        self.iter_mut().map(R::roll).sum()
    }
}

impl<R, N> Roll for (R, N)
where
    R: Roll,
    N: Into<usize> + Clone,
{
    fn roll(&mut self) -> usize {
        let (r, n) = self;
        (0..(n.clone().into())).map(|_| r.roll()).sum()
    }
}

use rand::distributions::*;
use rand::prelude::*;

type Weight = u16;

#[derive(Debug)]
pub struct Die {
    kind: DieKind,
    rng: ThreadRng,
}

impl From<&DieKind> for Die {
    fn from(kind: &DieKind) -> Self {
        let rng = Default::default();
        let kind = kind.clone();
        Self { kind, rng }
    }
}

impl Roll for Die {
    fn roll(&mut self) -> usize {
        match &self.kind {
            DieKind::Simple(distr) => distr.sample(&mut self.rng),
            DieKind::Weighted(distr, dots) => dots[distr.sample(&mut self.rng)],
        }
    }
}

#[derive(Debug, Clone)]
pub enum DieKind {
    Simple(Uniform<usize>),
    Weighted(WeightedIndex<Weight>, Vec<usize>),
}

impl DieKind {
    pub fn simple(sides: usize) -> Self {
        DieKind::Simple(Uniform::new_inclusive(1, sides))
    }

    pub fn weighted(sides: &[(usize, Weight)]) -> Self {
        let (dots, weights): (_, Vec<_>) = sides.into_iter().map(|&(d, w)| (d, w)).unzip();
        DieKind::Weighted(WeightedIndex::new(weights).unwrap(), dots)
    }
}

pub fn roll_threaded<I, D>(dice: I, total: usize) -> IntMap<usize, usize>
where
    I: IntoIterator<Item = (D, u8)>,
    D: Into<DieKind>,
{
    let parallel = num_cpus::get();
    let dice: Vec<(DieKind, u8)> = dice.into_iter().map(|(d, n)| (d.into(), n)).collect();

    let mut handles = Vec::with_capacity(parallel);
    for repetitions in
        (0..parallel).map(|n| total / parallel + if n < total % parallel { 1 } else { 0 })
    {
        let dice = dice.clone();
        let join_handle = std::thread::spawn(move || {
            let mut counter = IntMap::default();

            let mut dice: Vec<(Die, u8)> = dice.iter().map(|(d, n)| (Die::from(d), *n)).collect();
            for _ in 0..repetitions {
                let roll = (&mut dice).roll();
                let value: usize = counter.get(&roll).map(|v| *v).unwrap_or_default();
                counter.insert(roll, value + 1);
            }

            return counter;
        });

        handles.push(join_handle);
    }

    let mut final_counts = IntMap::default();
    for handle in handles {
        let counter = handle.join().unwrap();
        for (dots, count) in counter {
            let value: usize = final_counts.get(&dots).map(|v| *v).unwrap_or_default();
            final_counts.insert(dots, value + count);
        }
    }

    return final_counts;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
