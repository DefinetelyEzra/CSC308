#include <stdio.h>
#include <ctype.h>

int main() {
    char input[20];
    int isCharacter = 0;
    int isInteger = 0;
    int isFloat = 0;
    
    printf("Enter a character, float, or integer: ");
    scanf("%s", input);
    
    // Check if the input is a character, integer, or float
    if (isalpha(input[0]) && input[1] == '\0') {
        isCharacter = 1;
    } else {
        int i = 0, dotCount = 0;
        for (; input[i] != '\0'; i++) {
            if (input[i] == '.') dotCount++;
            else if (!isdigit(input[i])) break;
        }
        
        if (dotCount == 0 && i == strlen(input)) isInteger = 1;
        else if (dotCount == 1 && i == strlen(input)) isFloat = 1;
    }

    switch (isCharacter ? 1 : isInteger ? 2 : isFloat ? 3 : 0) {
        case 1: { // Character
            char ch = input[0];
            printf("You entered character: %c\n", ch);
            printf("ASCII code: %d\n", ch);
            printf("Size of character: %zu bytes\n", sizeof(ch));
            printf("Next four characters (multiples of 3):\n");
            for (int i = 1; i <= 4; i++) {
                printf("%c ", ch + i * 3);
            }
            printf("\n");
            break;
        }
        
        case 2: { // Integer
            int num;
            sscanf(input, "%d", &num);
            printf("You entered integer: %d\n", num);
            printf("Size of integer: %zu bytes\n", sizeof(num));
            printf("Next four integers (multiples of 3):\n");
            for (int i = 1; i <= 4; i++) {
                printf("%d ", num + i * 3);
            }
            printf("\n");
            break;
        }
        
        case 3: { // Float
            float num;
            sscanf(input, "%f", &num);
            printf("You entered float: %.2f\n", num);
            printf("Size of float: %zu bytes\n", sizeof(num));
            printf("Next four floats (multiples of 3):\n");
            for (int i = 1; i <= 4; i++) {
                printf("%.2f ", num + i * 3);
            }
            printf("\n");
            break;
        }
        
        default:
            printf("Invalid input. Please enter a character, integer, or float.\n");
            break;
    }

    return 0;
}