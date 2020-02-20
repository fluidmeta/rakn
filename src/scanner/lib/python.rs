use regex::Regex;
use std::{fmt, fs, io};
use walkdir::DirEntry;

#[derive(Builder, Clone)]
pub struct PythonPackage {
    name: String,
    version: String,
    lib_path: String,
}

impl PythonPackage {
    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }

    pub fn get_lib_path(&self) -> String {
        String::from(self.lib_path.as_str())
    }
}

pub struct PythonError {
    message: String,
}

impl From<io::Error> for PythonError {
    fn from(error: io::Error) -> Self {
        PythonError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for PythonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

pub fn scan(files: &Vec<DirEntry>) -> Result<Vec<PythonPackage>, PythonError> {
    let mut python_packages: Vec<PythonPackage> = vec![];
    let metadata_files: Vec<DirEntry> = files
        .clone()
        .into_iter()
        .filter(|e| {
            e.path()
                .to_str()
                .map(|s| s.ends_with(".dist-info/METADATA"))
                .unwrap_or(false)
        })
        .collect();

    for entry in metadata_files.into_iter() {
        // parse package version and name
        let contents = fs::read_to_string(entry.path().to_str().unwrap())?;
        let (name, version) = parse_py_metadata_file(contents.as_str());
        python_packages.push(PythonPackageBuilder::default()
            .name(String::from(name))
            .version(String::from(version))
            .lib_path(String::from(entry.path().to_str().unwrap()))
            .build()
            .unwrap());
    }

    Ok(python_packages)
}

fn parse_py_metadata_file(content: &str) -> (String, String) {
    lazy_static! {
        static ref RE_VERSION: Regex = Regex::new(r"(?m-i)^Version: (?P<version>[\d\\.]+)$").unwrap();
        static ref RE_NAME: Regex = Regex::new(r"(?m-i)^Name: (?P<name>[\w_-]+)$").unwrap();
    }

    let package_name = match RE_NAME.captures(content) {
        Some(s) => s.name("name").unwrap().as_str(),
        None => "",
    };

    let package_version = match RE_VERSION.captures(content) {
        Some(s) => s.name("version").unwrap().as_str(),
        None => "",
    };

    (String::from(package_name), String::from(package_version))
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// Tests
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_content_string() {
        let metadata_content = "
Metadata-Version: 2.1
Name: wheel
Version: 0.34.2
Summary: A built-package format for Python
Home-page: https://github.com/pypa/wheel
Author: Daniel Holth
Author-email: dholth@fastmail.fm
Maintainer: Alex Grönholm
Maintainer-email: alex.gronholm@nextday.fi
License: MIT
Project-URL: Documentation, https://wheel.readthedocs.io/
Project-URL: Changelog, https://wheel.readthedocs.io/en/stable/news.html
Project-URL: Issue Tracker, https://github.com/pypa/wheel/issues
Keywords: wheel,packaging
Platform: UNKNOWN
Classifier: Development Status :: 5 - Production/Stable
Classifier: Intended Audience :: Developers
Classifier: Topic :: System :: Archiving :: Packaging
Classifier: License :: OSI Approved :: MIT License
Classifier: Programming Language :: Python
Classifier: Programming Language :: Python :: 2
Classifier: Programming Language :: Python :: 2.7
Classifier: Programming Language :: Python :: 3
Classifier: Programming Language :: Python :: 3.5
Classifier: Programming Language :: Python :: 3.6
Classifier: Programming Language :: Python :: 3.7
Classifier: Programming Language :: Python :: 3.8
Requires-Python: >=2.7, !=3.0.*, !=3.1.*, !=3.2.*, !=3.3.*, !=3.4.*
Provides-Extra: test
Requires-Dist: pytest (>=3.0.0) ; extra == 'test'
Requires-Dist: pytest-cov ; extra == 'test'

wheel
=====

This library is the reference implementation of the Python wheel packaging
standard, as defined in `PEP 427`_.

It has two different roles:

#. A setuptools_ extension for building wheels that provides the
   ``bdist_wheel`` setuptools command
#. A command line tool for working with wheel files

It should be noted that wheel is **not** intended to be used as a library, and
as such there is no stable, public API.

.. _PEP 427: https://www.python.org/dev/peps/pep-0427/
.. _setuptools: https://pypi.org/project/setuptools/

Documentation
-------------

The documentation_ can be found on Read The Docs.

.. _documentation: https://wheel.readthedocs.io/

Code of Conduct
---------------

Everyone interacting in the wheel project's codebases, issue trackers, chat
rooms, and mailing lists is expected to follow the `PyPA Code of Conduct`_.

.. _PyPA Code of Conduct: https://www.pypa.io/en/latest/code-of-conduct/



";
        let (package_name, package_version) = parse_py_metadata_file(metadata_content);
        assert_eq!(package_name, "wheel");
        assert_eq!(package_version, "0.34.2");
    }
}
