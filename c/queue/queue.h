#ifndef queue_h
#define queue_h

#define ELEMENT_TYPE int

struct queue_record;
typedef struct queue_record *queue;

queue create_queue(int maxElements);
void dispose_queue(queue q);
int is_empty(queue q);

void enqueue(ELEMENT_TYPE x, queue q);
void dequeue (queue q);

void print_queue(queue q);
void print_head(queue q);
void print_tail(queue q);

#endif // !queue_h
