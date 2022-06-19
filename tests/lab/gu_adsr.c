#include <math.h>
#include "/usr/local/include/guile/3.0/libguile.h"

SCM
j0_wrapper (SCM x)
{
  return scm_from_double (j0 (scm_to_double (x, "j0")));
}

void
init_math_bessel ()
{
  scm_c_define_gsubr ("j0", 1, 0, 0, j0_wrapper);
}
