# data-record-rs

A procedural macro that auto-generates traits and implementations for your structs, enabling Java record–like behavior in Rust. It creates:
- A getter trait containing one method per named field.
- A constructor trait with a canonical constructor (default name `new`).

This allows you to keep struct fields private while providing read-only access and a standardized way to construct instances.

## Features

- **Getter Trait Generation:**  
  For each named field in your struct, a corresponding method is added to the generated getter trait.

- **Constructor Trait Generation:**  
  A constructor trait is generated with a method (default: `new`, customizable) that takes each field as an argument and returns an instance of the struct.

- **Custom Attributes:**  
  Fine-tune the generated code by applying custom attributes to:
    - The getter trait definition (`datarecord_getter_attr`)
    - The getter trait implementation (`datarecord_getter_impl_attr`)
    - The constructor trait definition (`datarecord_const_attr`)
    - The constructor trait implementation (`datarecordgit_const_impl_attr`)
    - The constructor method (`datarecord_const_impl_method_attr`)


## Requirements

- A Rust toolchain that supports procedural macros

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
data_record = "0.1.0"
