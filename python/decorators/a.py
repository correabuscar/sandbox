
#src: https://realpython.com/primer-on-python-decorators/

import functools

def debug(func):
    """Print the function signature and return value"""

    @functools.wraps(func)
    def wrapper_debug(*args, **kwargs):
        args_repr = [repr(a) for a in args]
        kwargs_repr = [f"{k}={v!r}" for k, v in kwargs.items()]
        signature = ", ".join(args_repr + kwargs_repr)
        print(f"Calling {func.__name__}({signature})")
        value = func(*args, **kwargs)
        print(f"{func.__name__!r} returned {value!r}")
        return value

    return wrapper_debug


if __name__ == '__main__':
    print("boo")
    @debug
    def make_greeting(name, age=None):
        if age is None:
            return f"Howdy {name}!"
        else:
            return f"Whoa {name}! {age} already, you are growing up!"
    make_greeting("Benjamin")
    make_greeting("Richard", age=112)
    make_greeting(name="Dorrisile", age=116)

    import math
    # Apply a decorator to a standard library function
    math.factorial = debug(math.factorial)

    def approximate_e(terms=18):
        return sum(1 / math.factorial(n) for n in range(terms))

    approximate_e(5)


