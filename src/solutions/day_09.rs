use crate::Answer;

#[derive(Debug, PartialEq, Copy, Clone)]
enum SlotKind {
    File { id: u32 },
    Free,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Slot {
    pub pos: usize,
    pub len: usize,
    pub kind: SlotKind,
}

impl Slot {
    pub fn is_free(&self) -> bool {
        match self.kind {
            SlotKind::File { .. } => false,
            SlotKind::Free => true,
        }
    }

    pub fn file_id(&self) -> u32 {
        match self.kind {
            SlotKind::File { id } => id,
            SlotKind::Free => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Slot> {
    let mut file_id = 0;
    let mut is_file = true;
    let mut data: Vec<Slot> = Vec::with_capacity(input.len());
    let mut pos = 0;
    for c in input.trim().chars() {
        let len: usize = (c as u32 - '0' as u32) as usize;
        if len > 0 {
            if is_file {
                data.push(Slot {
                    pos,
                    len,
                    kind: SlotKind::File { id: file_id },
                });
                file_id += 1;
            } else {
                data.push(Slot {
                    pos,
                    len,
                    kind: SlotKind::Free,
                });
            }
        }
        pos += len;
        is_file = !is_file;
    }
    data
}

pub fn part_a(input: &str) -> Answer {
    let mut input = parse_input(input);

    let mut left = 0;
    let mut rght = input.len() - 1;

    let mut ret: usize = 0;
    while left <= rght {
        // advance 'rght' ptr to next item
        while input[rght].is_free() {
            rght -= 1;
        }
        let pos_l = input[left].pos;
        let len_l = input[left].len;
        let len_r = input[rght].len;
        let id_r = input[rght].file_id();
        match input[left].kind {
            SlotKind::File { id: id_l } => {
                ret += id_l as usize * (pos_l * len_l + len_l * (len_l - 1) / 2);
                left += 1;
            }
            SlotKind::Free => {
                let len = len_l.min(len_r);
                ret += id_r as usize * (pos_l * len + len * (len - 1) / 2);
                if len_r == len_l {
                    // perfect fit
                    left += 1;
                    rght -= 1;
                } else if len_r <= len_l {
                    // right item fits: swallow item and decrease left free space
                    input[left].pos += len;
                    input[left].len -= len;
                    rght -= 1;
                } else {
                    // right item doesn't fit, move as much as we can and decrease item width
                    input[rght].len -= len;
                    left += 1;
                }
            }
        }
    }
    Answer::Number(ret as i64)
}

#[derive(Debug)]
struct FreeSpaceHolder {
    /// Keep track of free space. Each entry holds free space of a given size sorted in reverse by
    /// position.
    pub data: Vec<Vec<Slot>>,
}

impl FreeSpaceHolder {
    fn new(slots: &[Slot]) -> Self {
        let mut data: Vec<Vec<Slot>> = vec![vec![]; 10];
        for slot in slots.iter().rev() {
            if slot.is_free() {
                data[slot.len].push(*slot);
            }
        }
        Self { data }
    }

    fn use_freespace(&mut self, pos: usize, cap: usize) -> Option<usize> {
        let stack_to_use = (cap..10)
            .filter_map(|len| self.data[len].last())
            .filter(|slot| slot.pos < pos)
            .min_by_key(|slot| slot.pos)
            .map(|slot| slot.len);

        stack_to_use.map(|stack_idx| {
            let Slot { pos, len, kind: _ } = self.data[stack_idx].pop().unwrap();
            if len > cap {
                let new_len = len - cap;
                self.data[new_len].push(Slot {
                    pos: pos + cap,
                    len: new_len,
                    kind: SlotKind::Free,
                });
                // ensure it's sorted by bubbling up the new element
                let mut idx = self.data[new_len].len() - 1;
                while idx > 1 && self.data[new_len][idx - 1].pos < self.data[new_len][idx].pos {
                    self.data[new_len].swap(idx - 1, idx);
                    idx -= 1;
                }
            }
            pos
        })
    }
}

pub fn part_b(input: &str) -> Answer {
    let input = parse_input(input);
    let mut fsp = FreeSpaceHolder::new(&input);

    let mut ret: usize = 0;
    for rght in (0..input.len()).rev() {
        if input[rght].is_free() {
            continue;
        }
        let len = input[rght].len;
        let pos = fsp
            .use_freespace(input[rght].pos, len)
            .unwrap_or(input[rght].pos);
        ret += input[rght].file_id() as usize * (pos * len + len * (len - 1) / 2);
    }

    Answer::Number(ret as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(1928));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(2858));
    }
}
