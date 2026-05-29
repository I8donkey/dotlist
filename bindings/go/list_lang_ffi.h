#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct ListData ListData;

typedef struct CListData {
  struct ListData data;
} CListData;

struct CListData *list_data_new(void);

struct CListData *list_data_from_string(const char *content);

void list_data_free(struct CListData *ptr);

char *list_data_to_string(const struct CListData *ptr);

char *list_data_get(const struct CListData *ptr, const uintptr_t *indices, uintptr_t indices_len);

int list_data_append(struct CListData *ptr, uintptr_t index, const char *value);

int list_data_insert(struct CListData *ptr, uintptr_t index, uintptr_t position, const char *value);

int list_data_delete(struct CListData *ptr, uintptr_t index);

int list_data_replace(struct CListData *ptr, uintptr_t index, const char *new_value);

char *list_data_execute_command(struct CListData *ptr, const char *command);

void string_free(char *ptr);
