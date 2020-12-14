use num::Integer;

pub fn modulo<T>(a: T, b: T) -> T
where
	T: Copy + Integer + std::ops::Rem<Output = T>,
{
	((a % b) + b) % b
}
