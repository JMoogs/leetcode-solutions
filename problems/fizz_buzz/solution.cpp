class Solution {
public:
    std::vector<std::string> fizzBuzz(int n) {
	std::vector<std::string> v {};
	v.resize(n);
        for (int i = 1; i <= n; ++i) {
		if (i % 15 == 0) {
		   v[i - 1] = "FizzBuzz";
		} else if (i % 5 == 0) {
		   v[i - 1] = "Buzz";
		} else if (i % 3 == 0) {
		   v[i - 1] = "Fizz";
		} else {
		    v[i - 1] = std::to_string(i);
		}
	}
	return v;
    }
};
