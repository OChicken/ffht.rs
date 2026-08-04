"""
FFHT Modern Python 3 Compatible Version with Multi-Architecture Support.

This is a modified version of FFHT (https://github.com/FALCONN-LIB/FFHT)
with the following improvements:

1. Modern Python 3 Support: Fixed _ffht_3.c to build with Python 3.9+
   (original version only supported up to Python 3.8)

2. ARM NEON Support: Added ARM NEON SIMD implementation (fht_neon.c)
   for efficient execution on aarch64 platforms (e.g., Apple Silicon, ARM servers)

3. Rust Bindings: Added Rust FFI bindings for use in Rust projects

Supports both x86_64 (SSE/AVX) and aarch64 (NEON) architectures.

Original authors: Ilya Razenshteyn, Ludwig Schmidt
Modified by: OChicken
Repository: Part of ffht.rs wrapper project
"""

import os
import sys

try:
    import pypandoc
    long_description = pypandoc.convert('README.md', 'rst')
except(IOError, ImportError):
    long_description = open('README.md').read()

try:
    from setuptools import setup, find_packages, Extension
    from setuptools.command.build_ext import build_ext
except ImportError:
    sys.stderr.write('Setuptools not found!\n')
    raise


class build_ext_plain_suffix(build_ext):
    """Emit `ffht.so` instead of `ffht.cpython-314-x86_64-linux-gnu.so`.

    Setuptools appends the interpreter ABI tag (sysconfig's EXT_SUFFIX) so that
    builds for different Python versions can coexist in one directory.  We drop
    it because this .so is symlinked into several sibling projects and a stable
    name keeps those links from breaking on every Python upgrade.

    CPython still imports a bare `.so`: it is the last entry of
    importlib.machinery.EXTENSION_SUFFIXES.

    Trade-off: without the tag nothing stops a .so built for one Python from
    being loaded by another.  That does not fail cleanly -- it usually imports
    and then segfaults.  Rebuild after any Python upgrade.
    """

    def get_ext_filename(self, ext_name):
        return ext_name.replace('.', os.sep) + '.so'

try:
    import numpy as np
except ImportError:
    sys.stderr.write('NumPy not found!\n')
    raise

# Use Python 3 version (fixed for modern Python 3.9+)
# Original FFHT's _ffht_3.c only worked with Python 3.8 and below
arr_sources = ['_ffht_3.c', 'fht.c']

module = Extension('ffht',
                   sources=arr_sources,
                   extra_compile_args=['-march=native', '-O3', '-Wall', '-Wextra', '-pedantic',
                                       '-Wshadow', '-Wpointer-arith', '-Wcast-qual',
                                       '-Wstrict-prototypes', '-Wmissing-prototypes',
                                       '-std=c99', '-DFHT_HEADER_ONLY'],
                   include_dirs=[np.get_include()])

setup(name='FFHT',
      version='1.1',
      author='Ilya Razenshteyn, Ludwig Schmidt',
      author_email='falconn.lib@gmail.com',
      maintainer='OChicken',
      url='https://github.com/FALCONN-LIB/FFHT',
      description='Fast implementation of the Fast Hadamard Transform (FHT) - Python 3.9+ with x86_64 and aarch64 support',
      long_description=long_description,
      license='MIT',
      keywords='fast Fourier Hadamard transform butterfly SIMD SSE AVX NEON x86_64 aarch64 ARM',
      packages=find_packages(),
      include_package_data=True,
      ext_modules=[module],
      cmdclass={'build_ext': build_ext_plain_suffix})
