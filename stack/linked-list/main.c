#include <stdio.h>

#include "stack.h"


int main()
{
    stack s = create_stack();
    return 0;
}

void print_int_stack(stack s)
{
    node_ptr cur = s->next;
    while (!is_empty(s)) {
        printf("%d", )
    }
}