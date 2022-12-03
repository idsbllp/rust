selection=$1

# ${parameter//pattern/}
array=(${selection//\\s/})

# S=$(IFS=,; echo "${arr[*]}")
newDirectory=$(IFS=_; echo "${array[*]}")

newPath="src/${newDirectory}"
newFile="${newPath}/mod.rs"

echo "create new file: $newFile"

mkdir -p $newPath
touch $newFile

cat > $newFile <<EOF
pub fn print() {
  println!("================== ${selection} start ================");
  // TODO
  println!("\n================== ${selection} end ================");
}
EOF

