
#
# This file is the default set of rules to compile a Pebble project.
#
# Feel free to customize this to your needs.
#

import shutil
from waflib import Task, TaskGen
from waflib.TaskGen import extension

top = '.'
out = 'build'

TaskGen.declare_chain(
    name='rustc',
    rule='${RUSTC} -L ../lib/ -C target-cpu=cortex-m3 --target thumbv7m-none-eabi ${SRC} --emit=llvm-ir -o ${TGT}',
    ext_in='.rs',
    ext_out='.ll',)

TaskGen.declare_chain(
    name='llc',
    rule='${LLC} -mtriple=arm-none-eabi -relocation-model=pic -march=thumb -mattr=+thumb2 -mcpu=cortex-m3 --float-abi=soft --asm-verbose=false ${SRC} -o ${TGT}',
    ext_in='.ll',
    ext_out='.s',)

TaskGen.declare_chain(
    name='as',
    rule='${AS} ${ASFLAGS} -c ${SRC} -o ${TGT}',
    ext_in='.s',
    ext_out='.o',)

def options(ctx):
    ctx.load('pebble_sdk')

def configure(ctx):
    ctx.load('pebble_sdk')

def build(ctx):
    ctx.env.RUSTC = 'rustc'
    ctx.env.LLC = '../lib/llvm/Debug+Asserts/bin/llc'

    shutil.copy2('thumbv7m-none-eabi.json', out)

    ctx.load('pebble_sdk')

    ctx.pbl_program(source=ctx.path.ant_glob('src/*.(rs|c)'),
                    target='pebble-app.elf')

    ctx.pbl_bundle(elf='pebble-app.elf',
                   js=ctx.path.ant_glob('src/js/**/*.js'))
