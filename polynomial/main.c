#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "polynomial.h"


int main(void)
{
  struct people {
    int age;
    char *name;
  };

  struct people a = { 18, "Hello" };
  struct people *b = malloc(sizeof(struct people));

  memcpy(b, &a, sizeof(a));
  printf("%s\n", b->name);
  return 0;
}
