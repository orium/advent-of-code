#include <stdio.h>
#include <stdbool.h>

#define MAX_LINES 1000
#define MAX_COLUMNS 12

char numbers[MAX_LINES][MAX_COLUMNS + 1];
bool active[MAX_LINES];
int active_count;

char
get_most_common(int bit_index) {
    int count_zero = 0;
    int count_one = 0;

    for (int i=0; *numbers[i]; i++) {
        if (active[i]) {
            count_zero += numbers[i][bit_index] == '0';
            count_one += numbers[i][bit_index] == '1';
        }
    }

    return count_zero > count_one ? '0': '1';
}

char
get_least_common(int bit_index) {
    int count_zero = 0;
    int count_one = 0;

    for (int i=0; *numbers[i]; i++) {
        if (active[i]) {
            count_zero += numbers[i][bit_index] == '0';
            count_one += numbers[i][bit_index] == '1';
        }
    }

    return count_zero <= count_one ? '0': '1';
}

void
go(char (*criteria)(int)) {
    for (int bit_index = 0; numbers[0][bit_index]; bit_index++) {
        int most_common = criteria(bit_index);
        int any_active_index = -1;

#if 0
        printf("Most common in bit %d: %c\n", bit_index, most_common);
#endif

        for (int i = 0; *numbers[i]; i++) {
            if (active[i] && numbers[i][bit_index] != most_common) {
                active[i] = false;
                active_count--;
            }

            if (active[i]) {
                any_active_index = i;
#if 0
                printf("    keeping %s\n", numbers[i]);
#endif
            }
        }

        if (active_count == 1) {
            printf("%s\n", numbers[any_active_index]);
            break;
        }
    }
}

int
main(void) {
    for (int i = 0; scanf("%s", numbers[i]) == 1; i++) {
        active[i] = true;
        active_count++;
    }

    go(get_most_common);

    active_count = 0;

    for (int i = 0; *numbers[i]; i++) {
        active[i] = true;
        active_count++;
    }

    go(get_least_common);

    return 0;
}
