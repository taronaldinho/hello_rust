pub struct ToyVec<T> {
    elements: Box<[T]>,  // ボックス化されたスライス型で実データをヒープ領域に置く。
    len: usize, 
}

impl<T: Default> ToyVec<T> {

    // capacity が 0 の ToyVec を新規に作成する。
    pub fn new() -> Self {
        Self::with_capacity(0)  // 自身の関連関数 with_capacity を呼び出す。
    }

    // 指定した capacity を持つ ToyVec を作成する。
    pub fn with_capacity(capacity: usize) -> Self{
        Self {
            elements: Self::allocate_in_heap(capacity),  // 自身の関連関数 allocate_in_heap を呼び出す。
            len: 0,
        }
    }

    // 指定した size のヒープ領域を持つ Box<[T]> を返す。
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity -> usize {
        self.elements.len()
    }
}

fn main() {
    let t = ToyVec::new()
    println!("Hello, world!");
}
