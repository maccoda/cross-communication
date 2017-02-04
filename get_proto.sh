#/bin/bash

# Get the most up to date specification and generate the rust code for the
# protocol buffers

root_dir="../$(dirname $0)"
current_proj=$(basename $(pwd))

spec_dir="comms-spec"
proto_dir="proto"

if [[ ! -d $proto_dir ]]; then
    mkdir $proto_dir
fi

# Got to get this spec
cd $root_dir

if [[ ! -d "$spec_dir" ]]; then
    echo "Needing to clone it in"
    git clone git@gitlab.com:maccoda/comms-spec.git
    cd $spec_dir
else
    echo "Already exists just need to update"
    cd $spec_dir
    git pull
fi

# It the spec now get the proto files
cp -v *.proto ../$current_proj/$proto_dir/

cd ../$current_proj

echo "Generating the code from protocol buffer"
. gen_rust.sh
