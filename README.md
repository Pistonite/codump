# codump
![Build Badge](https://img.shields.io/github/actions/workflow/status/Pistonite/codump/rust.yml)
![Version Badge](https://img.shields.io/crates/v/codump)
![Docs Badge](https://img.shields.io/docsrs/codump)
![License Badge](https://img.shields.io/github/license/Pistonite/codump)
![Issue Badge](https://img.shields.io/github/issues/Pistonite/codump)

A straightforward tool for dumping code/comments from source files.

The tool uses regex for parsing comments and does not parse the language at all.
Therefore there's no requirement on directives or pragmas and it works on all source code,
given your codebase has a system for doc comments.

## Why
There are many existing tools that take comments in the codebase and turn them
into documentation. However, the quality of the generated documentation depends
fully on the quality of the comments. Most of the time you will end up with a generated
documentation where 90% of the pages are useless because the symbol is either not documented,
or the comment is empty/doesn't say anything, like:
```c#
/// <summary>
///
/// <summary>
/// <param name="input">the input</param>
public void DoSomething(string input) {
    ...
}
```
I found it to be more useful to reverse this process, where I *bring the code to the documentation*.
For example, my documentation can be a wiki page where I document not only the APIs and symbols,
but also the design principles, examples, etc... And the documentation tool can generate code snippets
to go along with the wiki page.

So that's the back story. This tool takes a file, a preset or custom comment syntax, and a search path to
find the symbol, then it prints out the code in a nice format ready to be embedded in a documentation page.

Obviously, just printing the code into the console doesn't make it documentation. I use [txtpp](https://github.com/Pistonite/txtpp)
for generating files with embedded commands.

## Contribution and Issues
Contribution and issues are welcome. This is my personal project and I have limited
bandwidth working on it, but I will take any suggestion/comment seriously.

## Install
As executable:
```
cargo install codump
```
As library:
```
cargo add codump
```
Add the `cli` feature if you need to parse config from CLI args
```
cargo add codump --features cli
```
## CLI Usage
```
A straightforward and flexible code/comment dump tool

Usage: codump [OPTIONS] <FILE> <SEARCH_PATH>...

Arguments:
  <FILE>
          The input file to parse

  <SEARCH_PATH>...
          The component search path
          
          Each path is a case-sensitive substring used to search for components at that level. The first line of the code after the doc comments is searched for the substring.

Options:
      --outer <OUTER>
          Outer single line comment regex

      --outer-start <OUTER_START>
          Outer multi line comment start regex

      --outer-end <OUTER_END>
          Outer multi line comment end regex

      --inner <INNER>
          Inner single line comment regex

      --inner-start <INNER_START>
          Inner multi line comment start regex

      --inner-end <INNER_END>
          Inner multi line comment end regex

  -i, --ignore <IGNORE>
          Pattern for lines that should be ignored

  -f, --format <FORMAT>
          Format for the output
          
          [default: summary]

          Possible values:
          - summary: Comments + abbreviated code
          - comment: Comment only format
          - detail:  Comment + all code

  -p, --preset <PRESET>
          Use a preset configuration
          
          If both presets and individual options are set, the individual options will override the corresponding part of the preset. The rest of the preset will still be used.

          Possible values:
          - rust:      Rust style
          - rust-java: Rust style for single line and Java/JS/TS style for multiline
          - python:    Python style

  -c, --context
          Print context
          
          Context is the parent components of the found component

  -C, --context-comments
          Print context comments
          
          Print the comments of the parents along with the context (implies --context)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Concept

### Outer comments
Outer comments is the comments placed before the thing it is for. Below is an example of an outer comment in rust style
```rust
/// Do something and returns the result
fn do_something() -> u32 {
    todo!();
}
```
Same outer comment in JS/TS style
```typescript
/**
    Do something and returns the result
*/
export function doSomething(): number {
    ...
}
```

In this commenting style, all of the code the comment is for is after the comment.

### Inner comments
Inner comments are comments placed inside the thing it is for. For example, module comments in rust.
```rust
//! This is a module to do things

pub fn do_thing_1() {
    ...
}

pub fn do_thing_2() {
    ...
}
```

Another example are doc comments in python
```python
def do_something():
    """Do something and returns the result"""
    ...

```

The distinction is that the "signature" of the code the comment is for is outside of the comment, and the details are after the comment

### Component
A component conceptually contain its comments and code. Its code may contain child components.

```rust
//! imagine this file is example.rs
//! this is the inner comment for that file

// code inside this component, but not a child component
use std;

/// a function is a child component
fn foo() -> u32 {
    ...
}

/// a nested module is a child component too
mod bar {
    //! inner comment (not sure if this is valid rust doc, but valid for this tool)

    /// child components can have children too
    fn biz() {
        ...
    }
}
```

This tool searches for component given a path. Once a component is found, the tool can print out its summary, comment, or implementation details.

## Parsing
The tool uses a simple parsing style based on lines and regex.

### Single and Multi-line Comments
The tool supports both single and multi-line comments for inner and outer comments. However, single and multi-line style cannot be mixes for the same component they are documenting

For example, you can do this:

```typescript
//! Example typescript file

/// Example 1
///
/// This function is documented with rust style single line comments
function foo() { ... }

/**
 * Example 2
 * 
 * C/Java/JS style
 */
function bar() { ... }
```

And this is invalid:
```typescript
/// This is a single line doc comment
/** 
which cannot be mixed with multi line
*/
function foo() { ... }
```

This is to keep the parser simple. In reality you wouldn't want to mix them for the same component you are documenting. Note that you can still mix the comments for the child components inside a component, which is what example 1 does.


### Component Structure
The parsing step turns list of lines into a component for searching. The component lines will be parsed assuming they are organized as following

1. Lines before the first line of inner comment. Ignored.
2. Inner comment, which can be:
    1. Consecutive lines that match the inner comment single line regex, or
    2. A line that matches the inner comment multi-line start regex, until and include the first line that matches the inner comment multi-line end regex
3. Lines before the first line of outer comment (of the children). Ignored
4. Repeat:
    1. Outer comment, which can be
        1. Consecutive lines that match the outer comment single line regex, or
        2. A line that matches the outer comment multi-line start regex, until and include the first line that matches the outer comment multi-line end regex
    2. Lines until the first line of the next outer comment.

### Nested Components
Nested children are parsed from the body lines (lines between outer comments) like so:

1. Determine the indentation by finding the first indented line. Only indentation by spaces or tabs are accepted.
2. Unindent the body lines. Remove the lines that are not indented.
3. Parse the result like a regular component

Note that this is not friendly to C++ namespace style where the things inside the namespace are commonly not indented
```c++

/// I am a namespace
namespace foo {

/// Why is this not indented
void do_something();

} // end namespace foo
```

Two (undesired) things will happen:
1. `do_something` will not be treated as a child component of `foo`, but in the same level as the namespace.
2. `} // end namespace foo` will be treated as part of `do_something`

For 2, we can workaround this by adding `^} // end namespace` to the pattern of lines to ignore.

For 1, we can still find the symbol by treating it as a component in the file.
However, if there are multiple functions with the exact same signature (except for the namespace) in the same file,
the tool will not be able to uniquely find it.

## Searching
The tool searches for a component by specifying a file and one of more search arguments.

Each search argument is used for searching the next nested component. If a nested component cannot be uniquely identified with the search term, the tool will error.

Since the tool uses comments to find the components, a component won't be found if it's not documented.

## Output Format
The tool supports 3 output formats for the component: `summary`, `comment` and `detail`.

Addtionally, you can use the `-k/-K` flags to print the parent(s) and the parent comments.

### Summary (default)
In summary mode, the outer and inner comments will be printed as-is.
Only lines in the body that do not have leading spaces will be printed. Indented blocks will be replaced by `...` with the same indentation 
as the first line of that block. This also applies to lines between the outer and inner comments.

### Comment
In comment mode, only the outer and inner comments are printed.

### Detail
In detail mode, all content of the component will be printed as-is.
