#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "polynomial.h"


int main(void)
{
  // 3X^5 + x^2
  struct node m2 = { 1, 2, 0 };
  struct node m1 = { 3, 5, &m2 };

  // 2X^4 + 3X^2
  struct node n2 = { 3, 2, 0 };
  struct node n1 = { 2, 4, &n2 };

  polynomial p1 = &m1;
  polynomial p2 = &n1;
  polynomial p3 = add(p1, p2);

  printf("p1 = ");
  print(p1);
  printf("p2 = ");
  print(p2);
  printf("p1 + p2 = ");
  print(p3);

  return 0;
}
