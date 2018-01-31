/*
 * Copyright (c) 2015 Brandon Sanderson
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE
 *
 */

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

///Type that should be ordered in
///the reverse order of any value it contains.
///
///This only implements the ordering traits
///when V does, and will never implement any traits
///other than PartialEq, PartialOrd, Eq, and Ord.
///Extract the value to use it in other ways.
///Note: revord_var.0 can be used to extract the value
pub struct RevOrd<V>(pub V);

impl<V> PartialEq for RevOrd<V> where V: PartialEq {
  fn eq(&self, other:&RevOrd<V>) -> bool {
    other.0 == self.0
  }
}

impl<V> PartialOrd for RevOrd<V> where V: PartialOrd {
  fn partial_cmp(&self, other:&RevOrd<V>) -> Option<Ordering> {
    (other.0).partial_cmp(&self.0)
  }
}

impl<V> Eq for RevOrd<V> where V: Eq {}

impl<V> Ord for RevOrd<V> where V: Ord {
    fn cmp(&self, other:&RevOrd<V>) -> Ordering {
      other.0.cmp(&self.0)
    }
}

impl<V> Deref for RevOrd<V> {
    type Target = V;

    fn deref(&self) -> &V {
        &self.0
    }
}

impl<V> DerefMut for RevOrd<V> {
    fn deref_mut(&mut self) -> &mut V {
        &mut self.0
    }
}


#[cfg(test)]
mod test {

  use super::RevOrd;

  #[test]
  fn int_test() {
    assert!(RevOrd(1) > RevOrd(2));
    assert!(RevOrd(1) < RevOrd(0));
  }

  #[test]
  fn deref_test() {
    let mut one = RevOrd(1);
    assert_eq!(1, *one);

    *one = 2;
    assert_eq!(2, *one);
  }
}
