import * as h from "./heap";

const heap: h.Heap = {
  elements: [-Infinity, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6, 7],
  size: 12
};

// h.maxHeapify(heap, 1);
h.buildMaxHeap(heap);
h.print(heap);

const heap2: h.Heap = {
  elements: [-Infinity, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6],
  size: 11
};

h.buildMaxHeap(heap2);
h.print(heap2);
