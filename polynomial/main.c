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


  return 0;
}
