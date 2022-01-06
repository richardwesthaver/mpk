project('mpk', ['rust','c','cpp'],
  version : '0.1.0',
  default_options : ['warning_level=3'])

rustc = find_program('rustc', version:'>=1.52')
cargo = find_program('cargo', version:'>=1.40')
cargo_wrapper = find_program('scripts/cargo_wrapper.py')

system = build_machine.system()
if system == 'windows'
  ext_dynamic = 'dll'
  ext_static = 'lib'
elif system == 'darwin'
  ext_dynamic = 'dylib'
  ext_static = 'a'
else
  ext_dynamic = 'so'
  ext_static = 'a'
endif

cc = meson.get_compiler('c')

jack_dep = cc.find_library('jack', required : false)
m_dep = cc.find_library('m', required : false)
csound_dep = cc.find_library('csound64', required : false)
exe = executable('mpk', 'src/mpk.c',
                 # install : true
                )

test('basic', exe)

