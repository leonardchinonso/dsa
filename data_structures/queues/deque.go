package queues

import (
	"fmt"

	ll "github.com/leonardchinonso/dsa/data_structures/linked_lists"
)

type Deque struct {
	dummyHead *ll.DLLNode
	tail      *ll.DLLNode
	size      int
	capacity  int
}

func NewDeque(capacity int) *Deque {
	return &Deque{
		dummyHead: ll.NewDLLNode(nil),
		capacity:  capacity,
	}
}

func (q *Deque) Size() int {
	return q.size
}

func (q *Deque) Enqueue(v interface{}) {
	if q.IsFull() {
		panic("queue is full!")
	}

	node := ll.NewDLLNode(v)

	if !q.IsEmpty() {
		q.tail.SetNext(node)
		node.SetPrev(q.tail)
	} else {
		q.dummyHead.SetNext(node)
		node.SetPrev(q.dummyHead)
	}

	q.tail = node
	q.size++
}

func (q *Deque) EnqueueLeft(v interface{}) {
	if q.IsFull() {
		panic("queue is full")
	}

	node := ll.NewDLLNode(v)

	if !q.IsEmpty() {
		tempNext := q.dummyHead.GetNext()
		q.dummyHead.SetNext(node)
		node.SetPrev(q.dummyHead)
		node.SetNext(tempNext)
		tempNext.SetPrev(node)
	} else {
		q.dummyHead.SetNext(node)
		node.SetPrev(q.dummyHead)
		q.tail = node
	}

	q.size++
}

func (q *Deque) Dequeue() interface{} {
	if q.IsEmpty() {
		panic("cannot dequeue empty queue")
	}

	node := q.tail
	q.tail = node.GetPrev()
	q.tail.SetNext(nil)

	if q.tail == q.dummyHead {
		q.tail = nil
	}

	q.size--
	return node.GetValue()
}

func (q *Deque) DequeueLeft() interface{} {
	if q.IsEmpty() {
		panic("cannot dequeue empty queue")
	}

	node := q.dummyHead.GetNext()
	tempNext := node.GetNext()
	q.dummyHead.SetNext(tempNext)
	if tempNext != nil {
		tempNext.SetPrev(q.dummyHead)
	}

	if q.tail == node {
		q.tail = nil
	}

	q.size--
	return node.GetValue()
}

func (q *Deque) PeekDeque() interface{} {
	if q.IsEmpty() {
		panic("cannot peek empty queue")
	}

	return q.tail.GetValue()
}

func (q *Deque) PeekDequeLeft() interface{} {
	if q.IsEmpty() {
		panic("cannot peek empty queue")
	}

	return q.dummyHead.GetNext().GetValue()
}

func (q *Deque) IsEmpty() bool {
	return q.size == 0
}

func (q *Deque) IsFull() bool {
	return q.size == q.capacity
}

func (q *Deque) Print() {
	fmt.Printf("[ ")
	curr := q.dummyHead.GetNext()
	for curr != nil {
		fmt.Printf("%v, ", curr.GetValue())
		curr = curr.GetNext()
	}
	fmt.Printf("]\n")
}

func TestDeque() {
	q := NewDeque(5)
	q.Enqueue(1)
	q.EnqueueLeft(2)
	q.Enqueue(3)
	q.EnqueueLeft(4)
	q.Enqueue(5)
	q.EnqueueLeft(6)
	q.Print()
	fmt.Printf("dequeued: %+v\n", q.Dequeue())
	q.Print()
	fmt.Printf("dequeued left: %+v\n", q.DequeueLeft())
	q.Print()
	fmt.Printf("size: %+v\n", q.Size())
	fmt.Printf("peek: %+v\n", q.PeekDeque())
	fmt.Printf("peekLeft: %+v\n", q.PeekDequeLeft())
}
