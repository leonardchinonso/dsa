/*
A class for creating nodes for a doubly linked list along with its methods
*/

package linked_lists

import "fmt"

type DLLNode struct {
	value interface{}
	prev  *DLLNode
	next  *DLLNode
}

func newDLLNode(value interface{}) *DLLNode {
	return &DLLNode{
		value: value,
	}
}

func (dn *DLLNode) GetValue() interface{} {
	return dn.value
}

func (dn *DLLNode) GetNext() *DLLNode {
	return dn.next
}

func (dn *DLLNode) GetPrev() *DLLNode {
	return dn.prev
}

func (dn *DLLNode) SetValue(value interface{}) {
	dn.value = value
}

func (dn *DLLNode) SetPrev(prev *DLLNode) {
	dn.prev = prev
}

func (dn *DLLNode) SetNext(next *DLLNode) {
	dn.next = next
}

func (dn *DLLNode) Print() {
	fmt.Printf("value: %+v, prev_node: %+v, next_node: %+v", dn.value, dn.prev, dn.next)
}

type DoublyList struct {
	dummyHead *DLLNode
	dummyTail *DLLNode
	length    int
	members   map[*DLLNode]bool
}

func NewDoublyList() *DoublyList {
	dl := &DoublyList{
		dummyHead: newDLLNode(nil),
		dummyTail: newDLLNode(nil),
		length:    0,
		members:   map[*DLLNode]bool{},
	}
	dl.dummyHead.SetNext(dl.dummyTail)
	dl.dummyTail.SetPrev(dl.dummyHead)
	return dl
}

func (dl *DoublyList) GetHead() *DLLNode {
	if dl.length == 0 {
		return nil
	}
	return dl.dummyHead.GetNext()
}

func (dl *DoublyList) GetTail() *DLLNode {
	if dl.length == 0 {
		return nil
	}
	return dl.dummyTail.GetPrev()
}

func (dl *DoublyList) SetHead(value interface{}) *DLLNode {
	node := newDLLNode(value)
	tempNext := dl.dummyHead.GetNext()

	dl.dummyHead.SetNext(node)
	node.SetNext(tempNext)

	tempNext.SetPrev(node)
	node.SetPrev(dl.dummyHead)

	dl.members[node] = true
	dl.length++

	return node
}

func (dl *DoublyList) AddNode(value interface{}) *DLLNode {
	node := newDLLNode(value)
	tempPrev := dl.dummyTail.GetPrev()

	node.SetPrev(tempPrev)
	dl.dummyTail.SetPrev(node)

	node.SetNext(dl.dummyTail)
	tempPrev.SetNext(node)

	dl.members[node] = true
	dl.length++

	return node
}

func (dl *DoublyList) RemoveNode(node *DLLNode) {
	if !dl.InList(node) {
		panic("cannot remove node because it is not in the list")
	}

	prev := node.prev
	next := node.next

	prev.SetNext(next)
	next.SetPrev(prev)

	delete(dl.members, node)
	dl.length--
}

func (dl *DoublyList) InList(node *DLLNode) bool {
	_, ok := dl.members[node]
	return ok
}

func (dl *DoublyList) Length() int {
	return dl.length
}

func (dl *DoublyList) Print() {
	if dl.length == 0 {
		return
	}

	curr := dl.dummyHead.GetNext()
	for curr != dl.dummyTail {
		fmt.Printf("value: %+v", curr.GetValue())
		curr = curr.GetNext()
		if curr != dl.dummyTail {
			fmt.Print(" <--> ")
		} else {
			fmt.Println()
		}
	}
}

func TestDoublyLinkedList() {
	list := NewDoublyList()
	list.AddNode(1)
	list.AddNode(2)
	list.AddNode(3)
	list.Print()
	list.SetHead(5)
	six := list.SetHead(6)
	list.AddNode(7)
	list.Print()
	list.RemoveNode(six)
	list.Print()
}
