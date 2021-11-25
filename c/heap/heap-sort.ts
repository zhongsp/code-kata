import * as h from './heap';

/**
 * The heap is represented as an Array.
 * And the index starts from 1.
 */
export function heapSort(heap: h.Heap): void {
  const { size, elements } = heap;
  const startIndex = 1;

  h.buildMaxHeap(heap);

  while(heap.size > 1) {
    // swap the first element and the last element
    [elements[startIndex], elements[heap.size]] = [elements[heap.size], elements[startIndex]];

    // got the reamining largest value
    heap.size--;

    // keep the heap
    h.maxHeapify(heap, startIndex);
  }

  // restore heap size
  heap.size = size;
}
