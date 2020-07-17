from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='pyprefetch-rs',
    version='0.1.0',
    rust_extensions = [
        RustExtension(
            'prefetch', 'Cargo.toml', binding=Binding.PyO3
        )
    ],
    zip_safe=False
)
