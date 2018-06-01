#include <stddef.h>
#include <stdio.h>

#include "list.h"

#define element_t int
int main(void) {
    //printf("%d", Node);
    list L = make_empty(NULL);
    append(1, L);
    append(2, L);
    append(3, L);
    append(4, L);
    print_list(L);
    printf("length = %d", length(L));
    getchar();
}
