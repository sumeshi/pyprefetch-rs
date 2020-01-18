from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='prefetch',
    rust_extensions = [
        RustExtension(
            'prefetch', 'Cargo.toml', binding=Binding.PyO3
        )
    ],
    zip_safe=False
)