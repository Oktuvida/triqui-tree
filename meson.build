project(
    'Triqui Tree',
    'cpp',
    version : '1.0.0',
    default_options : [
        'warning_level=3',
        'optimization=3', 
        'buildtype=release',
        'cpp_std=c++17',
        '''cpp_args=[
            "-march=native", 
            "-pipe", 
        ]''',
        'cpp_link_args="-Wl,-O1 -Wl,--as-needed"',
    ]
)

project_sources = []
main_source = []
inc = [include_directories('include')]

subdir('src')

project_bin_dep_libs = [
    algorithm_lib,
    board_lib,
    triqui_lib,
]

if host_machine.system() == 'Windows'
    bin_name = 'triqui-tree.exe'
else
    bin_name = 'triqui-tree'
endif

project_bin = executable(
    bin_name,
    main_source,
    include_directories: inc,
    link_with: project_bin_dep_libs,
)
