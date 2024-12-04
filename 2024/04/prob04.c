#include <stdio.h>
#include <stdlib.h>

char *read_input(char *file_name, int *rows, int *cols) {
  FILE *f = fopen(file_name, "r");
  if (f == NULL) {
    printf("Failed to open file %s\n", file_name);
    exit(1);
  }

  *rows = 1;
  *cols = 0;
  char c;
  while ((c = fgetc(f)) != EOF) {
    if (c == '\n') {
      (*rows)++;
    }
    if (*rows == 1) {
      (*cols)++;
    }
  }
  rewind(f);

  char *input = malloc(*rows * *cols);
  int i = 0;
  int j = 0;
  while ((c = fgetc(f)) != EOF) {
    if (c == '\n') {
      i++;
      j = 0;
    } else {
      input[i * (*cols) + j] = c;
      j++;
    }
  }
  fclose(f);
  return input;
}

int find_xmas(char *input, int rows, int cols, int i, int j, int dx, int dy) {
  if (i + 3 * dx >= rows || j + 3 * dy >= cols)
    return 0;
  if (i + 3 * dx < 0 || j + 3 * dy < 0)
    return 0;
  return input[(i + dx) * cols + (j + dy)] == 'M' &&
         input[(i + 2 * dx) * cols + (j + 2 * dy)] == 'A' &&
         input[(i + 3 * dx) * cols + (j + 3 * dy)] == 'S';
}

void part1(char *input, int rows, int cols) {
  int count = 0;
  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      if (input[i * cols + j] == 'X') {
        for (int dx = -1; dx <= 1; dx++) {
          for (int dy = -1; dy <= 1; dy++) {
            if (find_xmas(input, rows, cols, i, j, dx, dy))
              count++;
          }
        }
      }
    }
  }
  printf("part 1, count = %d\n", count);
}

void part2(char *input, int rows, int cols) {
  int count = 0;
  for (int i = 1; i < rows - 1; i++) {
    for (int j = 1; j < cols - 1; j++) {
      if (input[i * cols + j] == 'A') {
        char up_left = input[(i - 1) * cols + (j - 1)];
        char up_right = input[(i - 1) * cols + (j + 1)];
        char down_left = input[(i + 1) * cols + (j - 1)];
        char down_right = input[(i + 1) * cols + (j + 1)];
        int sm = up_left == 'S' && down_right == 'M';
        int ms = up_left == 'M' && down_right == 'S';
        int SM = up_right == 'S' && down_left == 'M';
        int MS = up_right == 'M' && down_left == 'S';
        if ((sm && SM) || (sm && MS) || (ms && SM) || (ms && MS)) {
          count++;
        }
      }
    }
  }
  printf("part 2, count = %d\n", count);
}

int main() {
  int rows = 0;
  int cols = 0;
  char *input = read_input("04.input", &rows, &cols);
  part1(input, rows, cols);
  part2(input, rows, cols);
  free(input);
}
