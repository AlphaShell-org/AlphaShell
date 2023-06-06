// TODO: add named args

let a = "5"; // local variable
export b = "6"; // global variable
let person = "John";
let c = pwd();
let people = ["John", "Peter"]; // array
let map = { foo: "bar", foo2: "baz" }; // associative array (dict)
let d = a == "2" ? "a" : "b";

/*
  block comment
*/

a = "10"; // throws warning - it does string concatenation

echo(a, b, c, d);

echo(people[1]); // zsh uses 1 indexing

echo(map["foo"]);
map["foo"] = "bar2";
echo(map["foo"]);

echo($(a + b)); // interpret as arithmetic

echo("a: ${a}"); // string interpolation
echo('a: ${a}'); // raw string

// tell the interpreter that a variable from
// outside of the current code exists
external "PATH";
echo(PATH);

// $n == n-th argument
// 0 is name, 1..9 are regular arguments
// works both for script args and function args
echo($0);
// @ is arguments array (excluding $0)
echo(@);

// newlines aren't significant, like in C-like languages
if a > 10 { echo("a is larger than 10"); }
if person != "Peter" { echo("${person} is not Peter"); }
if a <= 10 && b > 3 || person == "Jack" { echo("Complex condition satisfied"); }

// parenthesis for operation precedence, otherwise follows native zsh order
if (0==1 && 1==1) || 1==1 { echo("Should print"); }
if 0==1 && (1==1 || 1==1) { echo("Shouldn't print"); }

// export variable from the block scope
if true { echo("True"); export xyx = "xyz"; }
echo(xyx);

for a in 0..5 { echo(a); } // start..end[..step]
for a in 0..10..2 { echo(a); }
for word in ["hello", "world"] { echo(word); }
for person in people { echo(person); }

let x = "0";
while x < 10 {
  echo(x);
  $(x += 1); // without arithmetic, it'd do concatenation
}

// if command returns code 0, set list
// to the output and run the block
if let list = grep("let a", "demo.ash") {
  echo("Found something");
  echo(list);
} else {
  echo("Found nothing");
}

// while command is returning code 0,
// set var to it's output and run this code
while let var = grep("abcd", "file") {
  echo(var);
}

fn hello(name, age, cwd) {
  // same thing, the names are just aliases
  echo("Hello ${name} aged ${age}, we're in ${cwd}");
  echo("Hello ${1} aged ${2}, we're in ${3}");
}

hello("John", "25", c) | lolcat(); // pipe function output
let fb = "foobar" | sed("s/bar/baz/g"); // pipe value
let ab = $(a - b) | wc(-c); // flags get interpreted even without string
let ab2 = $(a - b) | wc(--chars); // long flags works as well

echo(fb);
echo(ab);
echo(ab2);

fn arrayFunction() {
  for a in @ {
    echo(a);
  }

  // return values work same as in zsh
  // 0 == success
  // 1..256 == error
  return 0;
}

arrayFunction("a", "b", "c", "d", "e");

let matched = "abcb";
if matched ~= "abc{2,5}" { // regex matching
  echo("matched");
} elif matched ~= "abc(cb)?" {
  echo("matched elif");
} else {
  echo("not matched");
}

firefox("google.com") &; // run as a daemon

import "test/test2.ash"; // transpile time import - similar to `#include`

source "another_folder/file.zsh", "/system/folder/file.zsh"; // runtime import, just like source in zsh
