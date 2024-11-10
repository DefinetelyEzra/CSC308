#include <stdio.h>

int main() {
    int experience, age;
    int salary;

    printf("Enter experience (1 for experienced, 0 for inexperienced): ");
    scanf("%d", &experience);

    // Get the input for age
    printf("Enter age: ");
    scanf("%d", &age);

    if (experience == 1) { // Experienced 
        if (age >= 40) {
            salary = 560000;
        } else if (age >= 30 && age < 40) {
            salary = 480000;
        } else if (age < 28) {
            salary = 300000;
        } else {
            salary = 0; // No salary specified for experienced between 28 and 30
        }
    } else { // Inexperienced
        salary = 100000;
    }
    
    if (salary > 0) {
        printf("The salary of the person is $%d\n", salary);
    } else {
        printf("No salary specified for the given experience and age.\n");
    }

    return 0;
}