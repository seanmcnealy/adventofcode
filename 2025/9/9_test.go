package main

import "testing"

func TestAbsInt(t *testing.T) {
	var got = absInt(-1)
	if got != 1 {
		t.Error("Expected 1")
	}
}

func TestIsInside(t *testing.T) {
	var path = [][2]int{
		{7, 1},
		{11, 1},
		{11, 7},
		{9, 7},
		{9, 5},
		{2, 5},
		{2, 3},
		//added
		{3, 3},
		{3, 1},
		{5, 1},
		{5, 3},
		//end-added
		{7, 3},
	}
	var got = isInsidePath(path, [2]int{0, 0})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{8, 6})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{8, 7})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{7, 6})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{6, 6})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{2, 6})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{1, 6})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{1, 5})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{1, 4})
	if got != false {
		t.Error("Expected false")
	}

	got = isInsidePath(path, [2]int{2, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{2, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{2, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{3, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{3, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{3, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{6, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{6, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{6, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{7, 1})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{7, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{7, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{7, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{7, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{8, 1})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{8, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{8, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{8, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{8, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 1})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 6})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{9, 7})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 1})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 6})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{10, 7})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 1})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 3})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 4})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 5})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 6})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{11, 7})
	if got != true {
		t.Error("Expected true")
	}

	got = isInsidePath(path, [2]int{3, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{4, 2})
	if got != true {
		t.Error("Expected true")
	}
	got = isInsidePath(path, [2]int{5, 2})
	if got != true {
		t.Error("Expected true")
	}

	got = isInsidePath(path, [2]int{3, 0})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{4, 0})
	if got != false {
		t.Error("Expected false")
	}
	got = isInsidePath(path, [2]int{5, 0})
	if got != false {
		t.Error("Expected false")
	}
}
