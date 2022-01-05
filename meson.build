project('mpk', ['c'],
  version : '0.1',
  default_options : ['warning_level=3'])

cc = meson.get_compiler('c')

pipewire_d = dependency('libpipewire-0.3')
spa_d = dependency('libspa-0.2')
m_d = cc.find_library('m', required : false)

exe = executable('dmw', 'dmw.c',
                 # install : true
                )

# pipewire tutorials
executable('pw-t1', 'misc/pw_tut/t1.c',
           dependencies: [pipewire_d])
executable('pw-t2', 'misc/pw_tut/t2.c',
           dependencies: [pipewire_d])
executable('pw-t3', 'misc/pw_tut/t3.c',
           dependencies: [pipewire_d])
executable('pw-t4', 'misc/pw_tut/t4.c',
           dependencies: [pipewire_d, spa_d, m_d])

test('basic', exe)

