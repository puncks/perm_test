# perm test
A rust implementation of comparative permutation testing, because python is not that fast

Input example: ` perm_test.test(amount, in_group, data) `.
The `in_group` variable must be a list of booleans of the same size as data, False if they're in group 0 and True if they're in group 1.
Only one dimensional datasets have been implemented.
