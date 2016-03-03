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

extern crate num;

use std::cmp::Ordering;
use num::Bounded;

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

impl<V: Bounded> Bounded for RevOrd<V> {
    fn min_value() -> RevOrd<V> {
        RevOrd(V::max_value())
    }
    
    fn max_value() -> RevOrd<V> {
        RevOrd(V::min_value())
    }
}

#[cfg(test)]
mod test {
  use super::num::Bounded;
  use super::RevOrd;

  #[test]
  fn int_test() {
    assert!(RevOrd(1) > RevOrd(2));
    assert!(RevOrd(1) < RevOrd(0));
  }

    #[test]
    fn int_bounds_test() {
        let max: RevOrd<i32> = Bounded::max_value();
        let min: RevOrd<i32> = Bounded::min_value();
        
        assert!(max > min);
    }
}
