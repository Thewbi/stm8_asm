//#include <stdio.h>

/* Print Fahrenheit-Celsius table for fahr = 0,20,...,300 */

int main()
{
    //float fahr, celsius;
    //float lower, upper, step;

    //int fahr, celsius;
    //int lower, upper, step;

    int fahr;
    int celsius;
    int lower;
    int upper;
    int step;

    lower = 0;   // Lower limit of temperature scale
    upper = 300; // Upper Limit
    step = 20;   // Step size

    celsius = lower;

    //printf("Celsius\tFahrenheit\n");

    while (celsius <= upper) {

        /* (0 degrees C 9/5) + 32 */

        //fahr = (celsius * (9.0 / 5.0)) + 32;

        fahr = (celsius * (9 / 5)) + 32;

        return fahr;

        //printf("%8.0f %10.2f\n", celsius, fahr);

        celsius = celsius + step;
    }

    return 17;
}
