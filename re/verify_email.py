import re
pattern = "[a-zA-Z0-9]+@[a-zA-Z]+\.(com|net|eu|edu)"
user_input = input()
if(re.search(pattern, user_input)):
    print("Valid Email!")
else:
    print("Invalid email!")
