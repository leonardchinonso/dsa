/*
A class for creating nodes for a single linked list along with its methods
*/

package linked_lists

import "fmt"

type SLLNode struct {
	value interface{}
	next  *SLLNode
}

func NewSLLNode(value interface{}) *SLLNode {
	return &SLLNode{
		value: value,
	}
}

func (sn *SLLNode) SetValue(value interface{}) {
	sn.value = value
}

func (sn *SLLNode) SetNext(next *SLLNode) {
	sn.next = next
}

func (sn *SLLNode) GetValue() interface{} {
	return sn.value
}

func (sn *SLLNode) GetNext() *SLLNode {
	return sn.next
}

func (sn *SLLNode) Print() {
	fmt.Printf("value: %+v, next_node: %+v", sn.value, sn.next)
}

type SinglyList struct {
	dummyHead *SLLNode
	tail      *SLLNode
	length    int
	members   map[*SLLNode]bool
}

func NewSinglyList() *SinglyList {
	return &SinglyList{
		dummyHead: NewSLLNode(nil),
		members:   map[*SLLNode]bool{},
	}
}

func (sl *SinglyList) GetHead() *SLLNode {
	if sl.length == 0 {
		return nil
	}
	return sl.dummyHead.GetNext()
}

func (sl *SinglyList) SetHead(value interface{}) *SLLNode {
	node := NewSLLNode(value)

	nextNode := sl.dummyHead.GetNext()
	if nextNode != nil {
		node.SetNext(nextNode)
	}

	sl.dummyHead.next = node

	sl.members[node] = true
	sl.length++

	return node
}

func (sl *SinglyList) AddNode(value interface{}) *SLLNode {
	newTail := NewSLLNode(value)

	if sl.tail == nil {
		sl.dummyHead.SetNext(newTail)
	} else {
		sl.tail.SetNext(newTail)
	}
	sl.tail = newTail

	sl.members[newTail] = true
	sl.length++

	return sl.tail
}

func (sl *SinglyList) Length() int {
	return sl.length
}

func (sl *SinglyList) Print() {
	if sl.length == 0 {
		return
	}

	curr := sl.dummyHead.GetNext()
	for curr != nil {
		fmt.Printf("value: %+v", curr.GetValue())
		curr = curr.GetNext()
		if curr != nil {
			fmt.Print(" --> ")
		} else {
			fmt.Println()
		}
	}
}

func TestSinglyLinkedList() {
	list := NewSinglyList()
	list.AddNode(1)
	list.AddNode(2)
	list.AddNode(3)
	list.Print()
	list.SetHead(5)
	list.SetHead(6)
	list.AddNode(7)
	list.Print()
}
