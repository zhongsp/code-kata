#ifndef stack_h
#define stack_h

#define ELEMENT_TYPE int


struct node;
typedef struct node *stack;

stack create_stack(void);
void make_empty(stack s);
void push(ELEMENT_TYPE element, stack s);
void pop(stack s);
ELEMENT_TYPE top(stack s);
int is_empty(stack s);
void dispose_stack(stack s);

void print_stack_int(stack s);

#endif // !stack_h
