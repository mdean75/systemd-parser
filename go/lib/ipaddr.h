#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Person {
  uint8_t age;
  const char *name;
} Person;

void parser(void);

struct Person new_person(void);
