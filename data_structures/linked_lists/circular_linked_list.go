package linked_lists

func NewCircularList() *DoublyList {
	cl := NewDoublyList()
	cl.dummyHead.SetPrev(cl.dummyTail)
	cl.dummyTail.SetNext(cl.dummyHead)
	return cl
}

func TestCircularLinkedList() {
	list := NewCircularList()

	if list.dummyHead.GetPrev() != list.dummyTail {
		panic("head.prev should be equal to tail")
	}

	if list.dummyTail.GetNext() != list.dummyHead {
		panic("tail.next should be equal to head")
	}
}
