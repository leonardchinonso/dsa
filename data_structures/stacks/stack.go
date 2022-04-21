package stacks

import "fmt"

type Stack struct {
	buf      []interface{}
	capacity int
	size     int
}

func NewStack(maxSize int) *Stack {
	return &Stack{
		buf:      make([]interface{}, maxSize, maxSize),
		capacity: maxSize,
		size:     0,
	}
}

func (s *Stack) GetSize() int {
	return s.size
}

func (s *Stack) Pop() interface{} {
	if s.size == 0 {
		panic("cannot pop from an empty stack")
	}
	last := s.buf[s.size-1]
	s.buf[s.size-1] = nil
	s.size--
	return last
}

func (s *Stack) Push(v interface{}) {
	if s.size == s.capacity {
		panic("stack is full")
	}
	s.buf[s.size] = v
	s.size++
}

func (s *Stack) Peek() interface{} {
	return s.buf[s.size-1]
}

func (s *Stack) IsEmpty() bool {
	return s.size == 0
}

func (s *Stack) IsFull() bool {
	return s.size == s.capacity
}

func (s *Stack) Print() {
	fmt.Printf("[ ")
	for _, el := range s.buf {
		if el != nil {
			fmt.Printf("%v, ", el)
		}
	}
	fmt.Printf("]\n")
}

func TestStack() {
	stack := NewStack(5)
	stack.Push(1)
	stack.Push(2)
	stack.Push(3)
	stack.Push(4)
	stack.Print()
	popped := stack.Pop()
	fmt.Printf("popped is: %v\n", popped)
	popped = stack.Pop()
	fmt.Printf("popped is: %v\n", popped)
	stack.Print()
	fmt.Printf("peek: %v\n", stack.Peek())
	fmt.Printf("is empty: %v\n", stack.IsEmpty())
	fmt.Printf("is full: %v\n", stack.IsFull())
	stack.Push(5)
	stack.Push(6)
	stack.Push(7)
	stack.Print()
}
