char** fizzBuzz(int n, int* returnSize) {
	*returnSize = n;
	char** arr = (char**) malloc(n * sizeof(char*));
	for (int i = 1; i <= n; ++i) {
		if (i % 15 == 0) {
			arr[i-1] = strdup("FizzBuzz");
		} else if (i % 5 == 0) {
			arr[i-1] = strdup("Buzz");
		} else if (i % 3 == 0) {
			arr[i-1] = strdup("Fizz");
		} else {
			char str[5];
			sprintf(str, "%d", i);
			arr[i-1] = strdup(str);
		}
	}
	return arr;
}
