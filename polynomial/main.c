#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "polynomial.h"


int main(void)
{
  // 3X^5 + x^2
  struct node m2 = { 1, 2, 0 };
  struct node m1 = { 3, 5, &m2 };

  // 2X^5 + 3X^1
  struct node n2 = { 3, 1, 0 };
  struct node n1 = { 2, 5, &n2 };

  polynomial p1 = &m1;
  polynomial p2 = &n1;
  polynomial p3 = add(p1, p2);

  puts("***************");
  print(p1);
  printf("+\n");
  print(p2);
  printf("=\n");
  print(p3);

  polynomial p4 = 0;
  polynomial p5 = add(p1, p4);
  
  puts("***************");
  print(p1);
  printf("+\n");
  print(p4);
  printf("=\n");
  print(p5);

  polynomial p6 = multiply(p1, p1);
  
  puts("***************");
  print(p1);
  printf("*\n");
  print(p1);
  printf("=\n");
  print(p6);

  puts("***************");
  polynomial p7 = multiply(p1, p2);
  print(p7);
  combine_like_terms(p7);
  print(p7);

  return 0;
}
