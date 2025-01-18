# need to compile exception_handeling.rs file and get librust_exceptions.so to work
import librust_exceptions

print(librust_exceptions.divide(10, 2))

try:
    print(librust_exceptions.divide(10, 0))
except ZeroDivisionError as e:
    print(f"Triggered exceptino from rust: {e}")
