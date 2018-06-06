#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "polynomial.h"


polynomial add(polynomial p1, polynomial p2)
{
    polynomial result = 0;
    node_ptr result_cur = result;

    node_ptr p1_cur = p1;
    node_ptr p2_cur = p2;

    while(p1_cur || p2_cur) {
        node_ptr result_node = malloc(sizeof(struct node));
        result_node->next = 0;

        if (p1_cur == 0 || p2_cur == 0) {
            if (p1_cur != 0 && p2_cur == 0) {
                memcpy(result_node, p1_cur, sizeof(struct node));
                p1_cur = p1_cur->next;

            } else if (p1_cur == 0 && p2_cur != 0) {
                memcpy(result_node, p2_cur, sizeof(struct node));
                p2_cur = p2_cur->next;
            }

        } else {
            if (p1_cur->exponent > p2_cur->exponent) {
                memcpy(result_node, p1_cur, sizeof(struct node));
                p1_cur = p1_cur->next;

            } else if (p1_cur->exponent < p2_cur->exponent) {
                memcpy(result_node, p2_cur, sizeof(struct node));
                p2_cur = p2_cur->next;

            } else {
                result_node->coefficient =
                    p1_cur->coefficient + p2_cur->coefficient;
                result_node->exponent = p1_cur->exponent;
                p1_cur = p1_cur->next;
                p2_cur = p2_cur->next;
            }
        }

        // result_node
        if (result == 0) {
            result = result_node;
            result_cur = result;

        } else {
            result_cur->next = result_node;
            result_cur = result_node;
        }
    }

    return result;
}

polynomial multiply(polynomial p1, polynomial p2)
{
    if (p1 == 0 || p2 == 0) {
        return 0;
    }

    polynomial result = 0;
    node_ptr result_cur = result;

    node_ptr p1_cur = p1;
    node_ptr p2_cur = p2;

    while (p1_cur) {
        p2_cur = p2;

        while(p2_cur) {
            node_ptr result_node = malloc(sizeof(struct node));
            result_node->coefficient = p1_cur->coefficient * p2_cur->coefficient;
            result_node->exponent = p1_cur->exponent + p2_cur->exponent;
            result_node->next = 0;

            if (result == 0) {
                result = result_node;
                result_cur = result;

            } else {
                result_cur->next = result_node;
                result_cur = result_node;
            }

            p2_cur = p2_cur->next;
        }

        p1_cur = p1_cur->next;
    }

    return result;
}

void combine_like_terms(polynomial p)
{
    node_ptr cur = p;

    node_ptr pre_check = 0;
    node_ptr check = 0;

    while (cur) {
        pre_check = cur;
        check = pre_check->next;

        while (check) {
            if (cur->exponent == check->exponent) {
                cur->coefficient += check->coefficient;
                pre_check->next = check->next;
                free(check);
                check = pre_check->next;
            } else {
                pre_check = check;
                check = pre_check->next;
            }
        }

        cur = cur->next;
    }
}

void print(polynomial p)
{
    while(p) {
        printf("%dX^%d", p->coefficient, p->exponent);
        p=p->next;

        if(p) {
            printf(" + ");
        }
    }
    printf("\n");
}
