package queues

import (
	"fmt"

	ll "github.com/leonardchinonso/dsa/data_structures/linked_lists"
)

type Seque struct {
	dummyHead *ll.SLLNode
	lastEntry *ll.SLLNode
	size      int
	capacity  int
}

func NewSeque(capacity int) *Seque {
	return &Seque{
		dummyHead: ll.NewSLLNode(nil),
		capacity:  capacity,
	}
}

func (q *Seque) IsEmpty() bool {
	return q.size == 0
}

func (q *Seque) IsFull() bool {
	return q.size == q.capacity
}

func (q *Seque) Enqueue(v interface{}) {
	if q.IsFull() {
		panic("queue is full!")
	}

	node := ll.NewSLLNode(v)

	if q.IsEmpty() {
		q.dummyHead.SetNext(node)
	} else {
		q.lastEntry.SetNext(node)
	}

	q.lastEntry = node
	q.size++
}

func (q *Seque) Dequeue() interface{} {
	if q.IsEmpty() {
		panic("cannot dequeue empty queue")
	}

	node := q.dummyHead.GetNext()
	q.dummyHead.SetNext(node.GetNext())
	q.lastEntry = node.GetNext()
	q.size--

	return node.GetValue()
}

func (q *Seque) Size() int {
	return q.size
}

func (q *Seque) PeekSeque() interface{} {
	if q.IsEmpty() {
		panic("queue is empty")
	}
	return q.dummyHead.GetNext().GetValue()
}

func (q *Seque) Print() {
	fmt.Printf("[ ")
	curr := q.dummyHead.GetNext()
	for curr != nil {
		fmt.Printf("%v, ", curr.GetValue())
		curr = curr.GetNext()
	}
	fmt.Printf("]\n")
}

func (q *Deque) PeekSeque() interface{} {
	if q.IsEmpty() {
		panic("cannot peek empty queue")
	}

	return q.tail.GetValue()
}

func TestSeque() {
	q := NewSeque(5)
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)
	q.Enqueue(4)
	q.Enqueue(5)
	q.Print()
	fmt.Printf("dequeued: %+v\n", q.Dequeue())
	q.Print()
	fmt.Printf("size: %+v\n", q.Size())
	fmt.Printf("peek: %+v\n", q.PeekSeque())
}
