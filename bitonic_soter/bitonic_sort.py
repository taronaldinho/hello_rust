def sort(x, up):    
    if len(x) == 1:        
        return x
    else:        
        mid_point = len(x) // 2
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)

        x1 = first + second
        
        return _sub_sort(x1, up)


def _sub_sort(x, up):
    if len(x) == 1:        
        return x
    else:
        x_ = _compare_and_swap(x, up)
        mid_point = len(x_) // 2
        first = _sub_sort(x_[:mid_point], up)
        second = _sub_sort(x_[mid_point:], up)

        return first + second


def _compare_and_swap(x: list, up):
    mid_point = len(x) // 2    
    for i in range(mid_point):               
        if (x[i] > x[mid_point + i]) == up:
            print("swap {} and {} -> ".format(x[i], x[mid_point + i]), end=None)
            x[i], x[mid_point + i] = x[mid_point + i], x[i]
            print(x)            
    return x


nums = [90, 4, 10, 11, 20, 21, 30, 110, 330, 58, 12, 58, 123, 3, 15, 39]
print(sort(nums, True))
