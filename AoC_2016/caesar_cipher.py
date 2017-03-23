def caesar_cipher(char, shift):
    a = ord(char)
    shift %= 26
    if a + shift >= ord('a') + 26:
        shifted_char = chr(a + shift - 26)
    else:
        shifted_char = chr(a + shift)
    return shifted_char

caesar_cipher("v", 343)
