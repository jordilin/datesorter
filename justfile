alias t:= test
alias tc:= test-coverage

test:
    cargo test

test-coverage:
    cargo llvm-cov --html
