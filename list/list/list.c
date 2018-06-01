#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

#include "list.h"


/**
 * Empty a list.
 */
list make_empty(list L)
{
    if (L == NULL) {
        L = malloc(sizeof(struct node));
        L->next = NULL;
    }

    position pos;
    while (L->next) {
        pos = L->next;
        L->next = pos->next;
        free(pos);
    }

    return L;
}

/**
 * Return true if L is empty.
 */
int is_empty(list L)
{
    return L->next == NULL;
}

/**
 * Return true if P is the last position in list L.
 * Parameter L is unused in this implementation.
 */
int is_last(position P, list L)
{
    return P->next == NULL;
}

/**
 * Return Position of X in L.
 * NULL if not found.
 */
position find(element_t X, list L)
{
    while (L->next) {
        if (L->next->element == X) {
            return L->next;
        } else {
            L = L->next;
        }
    }

    return NULL;
}

position last(list L)
{
    while (L->next) {
        L = L->next;
    }

    return L;
}

void append(element_t X, list L)
{
    position tail = last(L);

    position new_last = malloc(sizeof(struct node));
    if (new_last) {
        new_last->element = X;
        new_last->next = NULL;
        tail->next = new_last;
    }
}

int length(list L)
{
    int len = 0;
    while (L->next != NULL) {
        len++;
        L = L->next;
    }
    return len;
}

void print_list(list L)
{
    position pos = L->next;
    while (pos) {
        printf("%d -> ", pos->element);
        pos = pos->next;
    }
    printf("NULL \n");
}
