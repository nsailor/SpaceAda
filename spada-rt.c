#include <stdio.h>

float Int_To_Float(int x) {
	return (float)x;
}

int Float_To_Int(float f) {
	return (int)f;
}

void Print_Int(int x) {
	printf("-> %d\n", x);
}

int Read_Int() {
	int n;
	printf("Insert an integer > ");
	scanf("%d", &n);
	return n;
}

float Read_Float() {
	float f;
	printf("Insert a floating-point number > ");
	scanf("%f", &f);
	return f;
}

void Print_Float(float f) {
	printf("-> %f\n", f);
}

void Put_Char(int c) {
	printf("%c", (char)c);
}

void Put_Newline() {
	printf("\n");
}

void Main();

int main() {
	Main();
	return 0;
}
