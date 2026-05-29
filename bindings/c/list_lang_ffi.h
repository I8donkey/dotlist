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

uintptr_t list_data_len(const struct CListData *ptr);

int list_data_is_empty(const struct CListData *ptr);

char *list_data_get(const struct CListData *ptr, const uintptr_t *indices, uintptr_t indices_len);

char **list_data_get_array(const struct CListData *ptr, uintptr_t index, uintptr_t *out_len);

char **list_data_get_slice(const struct CListData *ptr,
                           uintptr_t index,
                           uintptr_t start,
                           uintptr_t end,
                           uintptr_t *out_len);

uintptr_t *list_data_find(const struct CListData *ptr, const char *pattern, uintptr_t *out_len);

uintptr_t *list_data_find_in_array(const struct CListData *ptr,
                                   uintptr_t index,
                                   const char *pattern,
                                   uintptr_t *out_len);

int list_data_append(struct CListData *ptr, uintptr_t index, const char *value);

int list_data_insert(struct CListData *ptr, uintptr_t index, uintptr_t position, const char *value);

int list_data_delete(struct CListData *ptr, uintptr_t index);

int list_data_replace(struct CListData *ptr, uintptr_t index, const char *new_value);

char *list_data_execute_command(struct CListData *ptr, const char *command);

int list_data_save_binary(const struct CListData *ptr, const char *path);

struct CListData *list_data_load_binary(const char *path);

void string_free(char *ptr);

void string_array_free(char **ptr, uintptr_t len);

void usize_array_free(uintptr_t *ptr, uintptr_t _len);
