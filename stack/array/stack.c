#include <stdlib.h>
#include <stdio.h>

#include "stack.h"

#define STACK_SIZE 1000
#define EMPTY_TOP_OF_STACK -1

typedef struct node {
    int top_of_stack;
    int capacity;
    ELEMENT_TYPE * elements;
} node;

// Create a header node.
stack create_stack(void)
{
    stack s = malloc(sizeof(node));
    s->top_of_stack = EMPTY_TOP_OF_STACK;
    s->capacity = STACK_SIZE;
    s->elements = malloc(sizeof(ELEMENT_TYPE) * STACK_SIZE);
    return s;
}

void make_empty(stack s)
{
    s->top_of_stack = EMPTY_TOP_OF_STACK;
}

int is_empty(stack s)
{
    return s->top_of_stack == EMPTY_TOP_OF_STACK;
}

void push(ELEMENT_TYPE element, stack s)
{
    s->elements[++s->top_of_stack] = element;
}

void pop(stack s)
{
    if (!is_empty(s)) {
        s->top_of_stack--;
    }
}

ELEMENT_TYPE top(stack s)
{
    if (!is_empty(s)) {
        return s->elements[s->top_of_stack];
    }
}

void dispose_stack(stack s)
{
    free(s->elements);
    free(s);
}

void print_stack_int(stack s)
{
    for (int i = s->top_of_stack; i > -1; i--) {
        printf("%d -> ", s->elements[i]);
    }
    puts("NULL");
}
