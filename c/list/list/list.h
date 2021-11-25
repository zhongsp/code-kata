#ifndef list_h
#define list_h

#define element_t int

struct node;
typedef struct node *node_ptr;
typedef node_ptr list;
typedef node_ptr position;

list make_empty(list L);
position find(element_t X, list L);
void delete(element_t X, list L);
void append(element_t X, list L);
int is_empty(list L);
int is_last(position P, list L);
void print_list(list L);
int length(list L);
position last(list L);

#endif // !list_h

struct node
{
    element_t element;
    position next;
};
