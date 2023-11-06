// https://leetcode.com/problems/seat-reservation-manager/

use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct SeatManager {
    pub seats: i32,
    pub free: BinaryHeap<Reverse<i32>>,
    pub occupied: HashSet<i32>,
    max_seats: i32,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            seats: 10,
            max_seats: n,
            free: BinaryHeap::from_iter((1..=10).map(|x| Reverse(x))),
            occupied: HashSet::new(),
        }
    }

    fn reserve(&mut self) -> i32 {
        if self.free.is_empty() {
            let reserve = self.seats + 1;
            self.free.append(&mut BinaryHeap::from_iter(
                (self.seats + 2..=self.seats + 10).map(|x| Reverse(x)),
            ));
            self.seats += 10;
            self.occupied.insert(reserve);
            return reserve;
        }

        let x = self.free.pop().unwrap();
        self.occupied.insert(x.0);
        return x.0;
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.occupied.remove(&seat_number);
        self.free.push(Reverse(seat_number));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut x = SeatManager::new(100000);
        let y = x.free.pop().unwrap().0;
        let y = x.free.pop().unwrap().0;
        let y = x.free.pop().unwrap().0;
        let y = x.free.pop().unwrap().0;
        println!("{}", y);
    }
}
