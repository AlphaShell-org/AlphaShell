let a = "5";
export b = 3 + 2 + 1; // throws warning - it does string concatenation
let person = "John";
let c = pwd();
let people = ["John", "Peter"]; // array
let map = { foo: "bar", foo2: "baz" }; // associative array (dict)
let d = a == "2" ? "a" : "b";

/*
  block comment
*/

a = "10";

echo(a, b, c, d);

echo(people[1]); // zsh uses 1 indexing

echo(map["foo"]);
map["foo"] = "bar2";
echo(map["foo"]);

echo($(a + b)); // interpret as arithmetic

echo("a: ${a}"); // string interpolation
echo('a: ${a}'); // raw string

if a > 10 { echo("a is arger than 10"); }
if person != "Peter" { echo("${person} is not Peter"); }
if a <= 10 && b > 3 || person == "Jack" { echo("Complex condition satisfied"); }

for a in 0..5 { echo(a); } // start..end[..step]
for a in 0..10..2 { echo(a); }
for word in ["hello", "world"] { echo(word); }
for person in people { echo(person); }

let x = 0;
while x < 10 {
  echo(x);
  $(x += 1); // without arithmetic, it'd do concatenation
}

fn hello(name, age, cwd) {
  echo("Hello ${name} aged ${age}, we're in ${cwd}");
}

hello("John", 25, c) | lolcat(); // pipe function output
let fb = "foobar" | sed("s/bar/baz/g"); // pipe value
let ab = $(a - b) | wc("-l"); // pipe value

echo(fb);
echo(ab);

fn arrayFunction() {
  for a in @ { // @ is args array
    echo(a);
  }

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
