#!/bin/bash
flatc -I $(pwd)/programs/flatbuffers_schema/ --rust --gen-object-api --gen-mutable --reflect-names -o ./crates/fb-types/src/ ./flatbuffers_schema/*.fbs
for file in ./crates/fb-types/src/*; do
    sed -e '/^#\[non_exhaustive\]/d' "$file" > temp_file && mv temp_file "$file"
done
rm ./crates/fb-types/src/temp_file