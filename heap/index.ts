import * as h from "./heap";

const heap: h.Heap = {
  elements: [-Infinity, 1, 2, 3, 4, 5, 6, 7],
  size: 7
};

// h.maxHeapify(heap, 1);
h.buildMaxHeap(heap);
h.print(heap);
