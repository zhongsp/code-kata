#include <stdlib.h>

#include "queue.h";


struct queue_record {
    int capacity;
    int front;
    int rear;
    int size;  // current elements in queue
    ELEMENT_TYPE *array;
};

queue create_queue(int maxElements)
{
    queue q = malloc(sizeof(struct queue_record));
    q->front = 0;
    q->rear = -1;
    q->size = 0;
    q->capacity = maxElements;
    q->array = malloc(sizeof(ELEMENT_TYPE) * maxElements);
}

void dispose_queue(queue q)
{
    free(q->array);
    free(q);
}

int is_empty(queue q)
{
    return q->size == 0;
}

void enqueue(ELEMENT_TYPE x, queue q)
{
    if (q->size < q->capacity) {
        q->size++;

        if (++q->rear >= q->capacity) {
            q->rear = 0;
        }

        q->array[q->rear] = x;
    }
}

void dequeue (queue q)
{
    if (is_empty(q)) {
        return;
    }

    q->size--;
    if (++q->front >= q->capacity) {
        q->front = 0;
    }
}
