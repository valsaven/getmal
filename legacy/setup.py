import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="getmal",
    version="1.0.0",
    author="Val Saven",
    author_email="val.saven@gmail.com",
    description="Returns the data of a particular anime list and status in JSON",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/valsaven/getmal",
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.6',
    keywords='myanimelist anime MAL json export',
    install_requires=[
        'beautifulsoup4',
    ],
)
