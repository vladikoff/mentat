// Copyright 2018 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use mentat_core::{
    TypedValue,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelResult {
    pub width: usize,
    pub values: Vec<TypedValue>,
}

impl RelResult {
    pub fn empty(width: usize) -> RelResult {
        RelResult {
            width: width,
            values: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn row_count(&self) -> usize {
        self.values.len() / self.width
    }
}

// Primarily for testing.
impl From<Vec<Vec<TypedValue>>> for RelResult {
    fn from(src: Vec<Vec<TypedValue>>) -> Self {
        if src.is_empty() {
            RelResult::empty(0)
        } else {
            let width = src.get(0).map(|r| r.len()).unwrap_or(0);
            RelResult {
                width: width,
                values: src.into_iter().flat_map(|r| r.into_iter()).collect(),
            }
        }
    }
}

pub struct SubvecIntoIterator {
    width: usize,
    values: ::std::vec::IntoIter<TypedValue>,
}

impl Iterator for SubvecIntoIterator {
    type Item = Vec<TypedValue>;
    fn next(&mut self) -> Option<Vec<TypedValue>> {
        let result: Vec<TypedValue> = (&mut self.values).take(self.width).collect();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl IntoIterator for RelResult {
    type Item = Vec<TypedValue>;
    type IntoIter = SubvecIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SubvecIntoIterator {
            width: self.width,
            values: self.values.into_iter(),
        }
    }
}
