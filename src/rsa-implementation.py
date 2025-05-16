from math import gcd 
import random 
import sympy


def generate_rsa_keys(): # by receiver before txn
    while True:
        p = random.randint(300, 10000)
        if sympy.isprime(p): break
    while True:
        q = random.randint(300, 10000)
        if sympy.isprime(q): break

    n = p * q   # n > msg to work
    r = (p - 1) * (q - 1) 

    # Public key (n,e), private key (n, d)
    for i in range(3, r):
        if gcd(i, r) == 1: # e relatively prime with r
            e = i
            break
    # d = e^-1 mod r
    d = pow(e, -1, r) 

    print(f"Private key (n,e) = ({n}, {e})")
    print(f"Public key (n,d) = ({n}, {d})")

    return n, e, d

    
# initial plan was to convert each letter to ascii nd concat them. (dum)
# It would have made msg hugee needing huuggerr n
# Instead encrypt/decrypt each letter's ascii seperately. join output later.


def rsa_encrypt(n: int, e: int, txt: str): #by sender with receiver's public key
    # txt^e % n. pow(base, exp, mod) uses binary exponentiation
    ct = [pow(ord(c), e, n) for c in txt] #list for each letter
    print(f"Encrypted ciphertext: {ct}")
    return ct

def rsa_decrypt(n: int, d: int, ct: int): #by receiver with their private key
    for c in ct:
        print(pow(c,d,n))
    mes = ''.join(chr(pow(c, d, n)) for c in ct)
    print(f"Decrypted message: {mes}")
    return mes


if __name__ == "__main__":
    n, e, d = generate_rsa_keys()
    ct = rsa_encrypt(n, e, "meow") #list
    rsa_decrypt(n, d, ct)