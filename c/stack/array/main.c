#include <stdio.h>

#include "stack.h"


int main()
{
    stack s = create_stack();
    push(1, s);
    push(2, s);
    push(3, s);
    print_stack_int(s);
    pop(s);
    print_stack_int(s);
    pop(s);
    print_stack_int(s);
    push(2, s);
    print_stack_int(s);
    dispose_stack(s);
    return 0;
}
