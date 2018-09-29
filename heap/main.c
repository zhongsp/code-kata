#include <stdio.h>

#include "heap.h"

int main(void)
{

  int heap[] = { 1, 4, 5, 2, 3, 2, 3 };
  int heap_size = 7;

  max_heapify(heap, heap_size, 0);

  for (int i = 0; i < heap_size; i++) {
    printf("%d ", heap[i]);
  }
  
  return 0;
}
