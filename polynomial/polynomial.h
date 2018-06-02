#ifndef polynomial_h
#define polynomial_h

typedef struct node *node_ptr;
typedef struct node *polynomial;

struct node
{
    int coefficient;
    int exponent;
    node_ptr next;
};

polynomial add(polynomial p1, polynomial p2);
polynomial multiply(polynomial p1, polynomial p2);

#endif  // !polynomial_h
