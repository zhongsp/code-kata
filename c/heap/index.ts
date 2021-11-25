import * as h from "./heap";
import { heapSort } from "./heap-sort";

const heap: h.Heap = {
  elements: [-Infinity, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6, 7],
  size: 12
};

h.buildMaxHeap(heap);
h.print(heap);

const heap1: h.Heap = {
  elements: [-Infinity, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6],
  size: 11
};

h.buildMaxHeap(heap1);
h.print(heap1);


const heap2: h.Heap = {
  elements: [-Infinity, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6, 7],
  size: 12
};

heapSort(heap2);
h.print(heap2);
