# REME - Rust SchEME

This is my take on a [Scheme](scheme.org) inspired rust
domain-specific language for creating syntax trees.

The work on this was in turn motivated by mato, my markdown
transformer framework. More specifically its parser, which creates
syntax trees that look like this:

```
Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Cat(Literal("This file contains only two"), LineBreak), Literal("paragraphs of text. It can be")), LineBreak), Literal("used to test verify some")), LineBreak), Literal("formatting properties.")), Cat(LineBreak, Paragraph)), Literal("The is the second paragraph.")), LineBreak), Literal("It contains a few sentences.")), LineBreak), Literal("Use mato to inspect, what")), LineBreak), Literal("groff source code resides.")), LineBreak)
```

Ugh - yes. Not only are these hard to read (in fact, I found them
*so* hard to read, I even added the possibility to export `dot`
files which can be rendered into nice diagrams using graphviz), but
they are not tail-recursive, but head recursive (I do not know, if
that is a thing, but it should be in my opinion, for symmetries
sake).

In REME one can tail-recursive lists very easily:

```
list!("this", "is", "a", "list")
```

Which yields the object graph:

```rust
Pair(Lit("this"), Pair(Lit("is"), Pair(Lit("a"), Pair(Lit("list"), Empty))))
```

And renders using the `Display` trait like this:

```
('this' 'is' 'a' 'list')
```

Ahhh ... much better (to be fair, in my mato example above we
*could* drop the `Cat`s and `Literals` and get a much conciser
display, but the nesting would stay, which can be hidden in REME, as
its representation is "well-formed" - meaning nested `Pair`s ending
with an `Empty` list marker node).

## Challenges

Appending to existing lists is a rather expensive operation in REME,
as the implementation has to walk over the existing list to the end
to locate the `Empty` node and replace it with a new `Pair`. And
even that is a lie, as these lists are immutable - so in addition to
the recursive descent, there is the effort of back-tracking up the
stack and constructing the new list as we go.

Here is the implementation:

```rust
pub fn append(a: Rc<List>, b: Rc<List>) -> Rc<List> {
    match *a {
        List::Empty() => b,
        _ => cons_rc(car_rc(Rc::clone(&a)), append(cdr_rc(a), b)),
    }
}
```

Nice and small, as these functional algorithms tend to be, but not
really efficient, I fear.
