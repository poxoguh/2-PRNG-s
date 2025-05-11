def bbs(iter) -> list[int]:
    p = 13 
    q = 11
    M = p*q
    result = [7] 
    for i  in range(0,iter-1):
        result.append((result[i]**2)% M )
    return result
print(bbs(10))
