import * as h from "./heap";

const heap: h.Heap = {
  elements: [-Infinity, 1, 4, 5, 2, 3, 2, 3],
  size: 7
};

h.maxHeapify(heap, 1);
h.print(heap);
