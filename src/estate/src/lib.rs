/*
    Estate Market:
    - Imagine a big Land broken into smaller blocks
    - A seqential number of x blocks is called a Estate.
    - A estate can be alloted or retained.
    - Two adjacent retained estate cannot be merged.
    - A Address will be given when a estate has been alloted.
    - And a Address is needed for a estate to be retained in future.
    - It may happen that a new market needs to be created by old data. 
      So combination of "register entry" and "retain" can be used for that.

    Note: Blocks will be assigned from 0.

    There are two types of Estate
    - Residentail: It is of fixed size.
    - Farming. It is of dynamic size on demand
*/

mod def;
mod farming;
mod residential;
pub use def::WholeNumber;
pub use farming::Farming;
pub use residential::Residential;
