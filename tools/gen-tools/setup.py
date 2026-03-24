from setuptools import setup, find_packages

setup(
    name="gen-tools",
    version="0.1.0",
    description="Tool generation for oline .team",
    author="Oline Team",
    packages=find_packages(),
    install_requires=[
        "click",
        "pyyaml",
        "rich",
    ],
    entry_points={
        'console_scripts': [
            'gen-tools=gen_tools.cli:main',
        ],
    },
)
