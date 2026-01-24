# XML - Extensible markup language

## Basics

- It is a **text format** for **structured data** that is **self-describing** and used for both machines and humans.
- XML represents data as a **tree** where everything is either an **element**, an **attribute** or **text**.

- Every XML must HAVE **exactly one root element**

This is an invalid structure:

```
<id>1</id>
<status>ok</status>
```

## Attributes

- Attributes live inside the start tag and **describe properties**:

```
<order status="confirmed">
```

**Rules**:

- Always `name="value"`
- Values must be quoted
- Attributes !== elements

_Bad practice_:

```
<order status="confirmed" date="2024-01-01" price="10"/>
```

_Better_:

```
<order status="confirmed">
    <date>2024-01-01</date>
    <price>10</price>
</order>
```

## XML declaration

These are instructions to the parser, not an element!

```
<?xml version="1.0" encoding="UTF-8"?>
```

It tells XML version and character encoding.

## Namespaces

- It tells the parser which **vocabulary** an attribute or an element belongs to. It tells **what it means**.

In the system with payment and order systems this solves ambiguity:

```
<ord:status>CONFIRMED</ord:status>
<pay:status>PAID</pay:status>
```
