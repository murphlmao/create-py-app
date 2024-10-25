# libraries
from setuptools import setup, find_packages
from distutils.util import convert_path


metadata = {}
ver_path = convert_path('{{ name }}/metadata.py')
with open(ver_path) as ver_file:
    exec(ver_file.read(), metadata)


setup(
    name=metadata["__title__"],
    version=metadata["__version__"],
    author=metadata["__author__"],
    author_email=metadata["__author_email__"],
    description=metadata["__description__"],
    #long_description=open("README.md").read(),
    #long_description_content_type="text/markdown",
    entry_points = {
        'console_scripts' : [
            'run_{{ name }} = {{ name }}:main'
        ]
    },
    python_requires=metadata["__python_requires__"],
    classifiers=metadata["__classifiers__"],
    install_requires=metadata["__dependencies__"],
    packages=find_packages(),

)