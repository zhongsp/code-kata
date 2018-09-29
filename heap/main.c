#include "heap.h"

int main(void)
{

  int heap0[] = { 1, 4, 5, 2, 3, 2, 3 };
  int heap_size0 = 7;
  max_heapify(heap0, heap_size0, 0);
  print_heap(heap0, heap_size0);
  // output: 5 4 3 2 3 2 1

  int heap1[] = { 16, 4, 10, 14, 7, 9, 3, 2, 8, 1 };
  int heap_size1 = 10;
  max_heapify(heap1, heap_size1, 1);
  print_heap(heap1, heap_size1);
  // output: 16 14 10 8 7 9 3 2 4 1

  return 0;
}
