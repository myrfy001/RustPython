#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void *rpy_compile_code(void *vm, const char *code);
void *rpy_new_scope_with_builtins(void *vm);
void rpy_run_code_obj(void *vm, void *code_obj, void *scope);
void *rpy_vm_new(void);

void test_main(void);
