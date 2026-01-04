# Design
see [docs](docs)

# Examples

## Hello world

```
let count: int = 0;
let msg: string = "hi";

print(123.4567);
print(true);
print(msg);

fn main() {
    print("HELLOWORLD!");
}
```

## expression
```
print(1 + 2 * 3 + 5);
print(1 + 2 * (3 + 5));
```
## simple loop

```
let count: int = 0;
while count < 5 {
    print(count + 42);
    count = count + 1;
}
print("count:");
print(count);
```

## function call & return value

```
fn fint() {
    print("fint");
    return 1;
}
fn fstring() {
    print("fstring");
    return "hello";
}

fn fmultilayer1() {
    return 42;
}

fn fmultilayer2() {
    return fmultilayer1()+1;
}

fn fmultilayer3() {
    return fmultilayer2()+2;
}

fn main() {
    print(fint());
    print(fstring());
    print(fmultilayer3());
}
```

## function call with args

```
fn fint(a int) {
    print("fint");
    print(a);
}
fn fintint(a int, b int) {
    let x : int = 0;
    print("fintint");
    print(a);
    print(b);
}
fn fstring(a string) {
    print("fstring");
    print(a);
}
fn fstringstring(a string, b string) {
    let x : string = "";
    print("fstringstring");
    print(a);
    print(b);
}
fn main() {
    fint(111);
    fintint(222, 233);
    fstring("s0");
    fstringstring("s1", "s2");
}
```

## Cooperate with dkv db server

```
let msg: string = "hi";
print(msg);

command("SET A xxx");
let r: string = command("GET A");
print(r);
```
