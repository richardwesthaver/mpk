#include <string.h>
#include <unistd.h>
#include <float.h>
#include <limits.h>
#include <ctype.h>
#include <time.h>
#include <scheme.h>

#define str_eq(X,Y) (!strcmp((X),(Y)))

FILE *open_file(char *fname) {
    if (str_eq(fname, "-") || str_eq(fname, "--")) {
        return stdin;
    }
    return fopen(fname, "r");
}

int main(int argc, char **argv) {
  FILE *fin = NULL;
  char *executable_name = argv[0];
  char *file_name = "init.scm";
  int res;
  int isfile = 1;
  scheme *sc = scheme_init_new();
  if (!scheme_init(sc)) {
    fprintf(stderr, "init failed\n");
    return 2;
  }
  scheme_set_input_port_file(sc, stdin);
  scheme_set_output_port_file(sc, stdout);
  fin = open_file(file_name);
  scheme_load_file(sc, fin);
  scheme_load_named_file(sc, stdin, "-");
  scheme_deinit(sc);
  return 1;
}

