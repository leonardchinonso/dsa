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
	q := &Seque{
		dummyHead: ll.NewSLLNode(nil),
		capacity:  capacity,
	}
	return q
}

func (q *Seque) Enqueue(v interface{}) {
	if q.size == q.capacity {
		panic("queue is full!")
	}

	node := ll.NewSLLNode(v)

	if q.size == 0 {
		q.dummyHead.SetNext(node)
	} else {
		q.lastEntry.SetNext(node)
	}

	q.lastEntry = node
	q.size++
}

func (q *Seque) Dequeue() *ll.SLLNode {
	if q.size == 0 {
		panic("cannot dequeue from empty queue")
	}

	node := q.dummyHead.GetNext()
	q.dummyHead.SetNext(node.GetNext())
	q.lastEntry = node.GetNext()
	q.size--

	return node
}

func (q *Seque) Size() int {
	return q.size
}

func (q *Seque) Peek() *ll.SLLNode {
	return q.dummyHead.GetNext()
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

func TestSeque() {
	q := NewSeque(5)
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)
	q.Enqueue(4)
	q.Enqueue(5)
	q.Print()
	fmt.Printf("dequeued: %+v\n", q.Dequeue().GetValue())
	q.Print()
	fmt.Printf("size: %+v\n", q.Size())
	fmt.Printf("peek: %+v\n", q.Peek())
}
