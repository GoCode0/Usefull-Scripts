import re
pattern = "(\d\d\d)-(\d\d\d)-(\d\d\d\d)"
new_pattern = r"\1\2\3"
user_input = input()
new_user_input = re.sub(pattern, new_pattern, user_input)
print(new_user_input)
