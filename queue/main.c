#include "queue.h"


int main(void)
{
  queue q = create_queue(10);

  enqueue(1, q);
  print_queue(q);
  print_head(q);
  print_tail(q);

  enqueue(2, q);
  print_queue(q);
  print_head(q);
  print_tail(q);

  dequeue(q);
  print_queue(q);
  print_head(q);
  print_tail(q);

  return 0;
}
