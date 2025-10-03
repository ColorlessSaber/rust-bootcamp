import ffi_rust_from_python as word_counter

def main():
    result = word_counter.sum_as_string(1,2) # throws a false warning. Code will still run.
    print(f"Result for sum_as_string: {result}")

    result = word_counter.count_words("in rust we trust rust or bust go for rust") # throws a false warning. Code will still run.
    print(f"Result for count_words: {result}")

if __name__ == '__main__':
    main()