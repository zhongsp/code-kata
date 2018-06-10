#include <stdlib.h>
#include <stdio.h>

#include "stack.h"

/**
 * Stack implementation is a linked list with a header.
 */

struct node {
    ELEMENT_TYPE element;
    node_ptr next;
};

// Create a header node.
stack create_stack(void)
{
    stack s = malloc(sizeof(struct node));
    s->next = 0;
    return s;
}

void make_empty(stack s)
{
    while (!is_empty(s)) {
        pop(s);
    }
}

int is_empty(stack s)
{
    return s->next == 0;
}

void push(ELEMENT_TYPE element, stack s)
{
    node_ptr new_node = malloc(sizeof(struct node));
    new_node->element = element;
    new_node->next = s->next;
    s->next = new_node;
}

void pop(stack s)
{
    if (!is_empty(s)) {
        node_ptr tmp_node = s->next;
        s->next = tmp_node->next;
        free(tmp_node);
    }
}

ELEMENT_TYPE top(stack s)
{
    if (!is_empty(s)) {
        return s->next->element;
    } else {
        return 0;
    }
}

void dispose_stack(stack s)
{
    make_empty(s);
    free(s);
}

void print_stack_int(stack s)
{
    node_ptr cur = s->next;
    while (cur) {
        printf("%d -> ", cur->element);
        cur = cur->next;
    }
    puts("NULL");
}
