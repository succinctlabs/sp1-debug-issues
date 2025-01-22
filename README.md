# sp1-debug-issues

A forkable repository to report SP1 issues.

## Steps

First, please check if your issue isn't one in the [common issues](https://docs.succinct.xyz/docs/developers/common-issues) page in our docs.

If not, please follow these steps:

* Fork this repo to your local machine
* Please prepending `SP1_DUMP=1` when running your normal program/script, it will output the `program.bin` and `stdin.bin` files to the current directory.
  Please note these 2 files are ony generated when generating proofs (not in execution only mode).
* Then copy them to this repository
* Run the following commands in sequence:
 
## Go through these commands

SP1 can be used in 3 modes: CPU, GPU and on the [Prover Network](https://docs.succinct.xyz/docs/generating-proofs/prover-network). Knowing which mode you are using,
and if it works in another mode will help us debug your issue.
 
* If you have access to the Prover Network,
  try [running your progam](https://docs.succinct.xyz/docs/generating-proofs/prover-network/usage) on it and see if it works. 
* If you have a [supported GPU](https://docs.succinct.xyz/docs/generating-proofs/hardware-acceleration/cuda),
  try running your program by prepending `SP1_PROVER=gpu` to you command to test that proving on your GPU works.
* If you issue is either when using your GPU or the Prover Network,
  try running your program by prepending `SP1_PROVER=cpu` to you command to test that proving on your CPU works.

And then report the command you ran and their output in the section below.

## Describe command for reproducible issue
 
**Please list the command here where you run into an issue**
 
* List the command: ...
* Write the terminal output: ...
* Write machine specs: ...