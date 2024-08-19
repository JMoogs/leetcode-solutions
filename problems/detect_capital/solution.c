bool detectCapitalUse(char* word) {
	int all_upper = 1;
	int all_lower = 1;
	int i = 1;
	while (*(word + i) != '\0') {
		isupper(*(word + i)) ? (all_lower = 0) : (all_upper = 0);
		if (all_lower == all_upper) { return 0; };
		++i;
	}
	return all_lower || (isupper(*word) && all_upper);
}
