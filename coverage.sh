wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz &&
    tar xzf master.tar.gz &&
    cd kcov-master &&
    mkdir build &&
    cd build &&
    cmake .. &&
    make &&
    make install DESTDIR=../../kcov-build &&
    cd ../.. &&
    rm -rf kcov-master &&
    for file in target/debug/zeus_compiler-*[^\.d]; do mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done &&
    for file in target/debug/examples/*[^\.d]; do mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done &&
    for file in target/debug/*test*[^\.d]; do mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done &&
    bash <(curl -s https://codecov.io/bash) &&
echo "Uploaded code coverage"
