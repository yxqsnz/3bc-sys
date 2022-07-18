#!/usr/bin/env bash
goto() {
  pushd "$1" > /dev/null
}
end() {
  popd > /dev/null
}
GAP="   "
echo ":: Updating 3bc-sys"

echo "${GAP}=> Fetching current 3bc"
git submodule update --init --recursive

echo "${GAP}=> Fetching latest tab..."

goto "./3bc-lang/" 
  git fetch --tags
  tag=$(git describe --tags `git rev-list --tags --max-count=1`)
  echo "${GAP}-> Latest tag: ${tag}"
  git checkout $tag
end

git add -A
git commit -m 'ci(auto): update 3bc-sys'
git push
