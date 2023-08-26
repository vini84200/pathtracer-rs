
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

type ColorF32 = Color<f32>;
type ColorU8 = Color<u8>;