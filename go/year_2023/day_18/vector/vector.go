package vector

type Vector [2]int

func UpVector() Vector    { return [2]int{0, -1} }
func LeftVector() Vector  { return [2]int{-1, 0} }
func RightVector() Vector { return [2]int{1, 0} }
func DownVector() Vector  { return [2]int{0, 1} }

func (self Vector) Add(other Vector) Vector {
	return [2]int{self[0] + other[0], self[1] + other[1]}
}

func (self Vector) Scale(n int) Vector {
	return [2]int{self[0] * n, self[1] * n}
}
