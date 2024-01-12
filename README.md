# Reading Note of RfR

## Progress

1. 2024-01-11, ![](https://geps.dev/progress/10)
1. 2024-01-12, ![](https://geps.dev/progress/12)

## FIX

1. Listing 1-10, it is not right, here `fn next(&self)` should be `fn next(&mut self)`

## Memory

For traditional programming languages like `C`, __pass by value__ is the default behavior, which `Copy` trait in Rust does in fact. But for value whose type does not implement Copy, Rust `move` its value, in another word, consuming it, which means it can never been used by the same variable name it bound to before. It is the key difference.

### TODO

1. memory-mapped registers
2. nonvolatile RAM
3. interior mutability, `unsafeCell`; These normally fall into two categories: 
4. Listing 1-10 code, thing about multiple lifetimes
5. `false sharing` in concurrent
6. `Vtable` struct
7. `Object safe`, __To be object-safe, none of a trait’s methods can be generic or use the Self type__

### Memory Leak

Memory leak do things in Rust, and call `leak` on `Box` type gives you static reference to memory area in runtime

> However, there are some cases where you explicitly want to leak memory. For example, say you have a read-only configuration that the entire program should be able to access. You can allocate that on the heap and explicitly leak it with Box::leak to get a 'static reference to it.

### Static References

1. It is just an annotation for the compiler to optimize code
2. It is not neccessarily point to a __static memory region__ of process memory, as the _memory leak_ part above describes

### `const` vs `static`

`const` is more like _C_ define, it is a compile time computable value handled by the compiler, it is a concept more about __value__, but not __address__. There is a concept named `comptime` in _Zig Programming Langue_

`static` is related to __lifetime of value__, it implies the value is alive when the process exists, it can be computed during runtime, also compile time.

### Drop Order

1. like `defer` in _golang_ or _Zig_, reverse order
2. nested type, refering to things like `vector`, drop in __source code order__; explain below

> Now, we could have the same behavior for nested values, like the values in a tuple, array, or struct, but that would likely surprise users. If you constructed an array that contained two values, it’d seem odd if the last element of the array were dropped first. The same applies to tuples and structs, where the most intuitive behavior is for the first tuple element or field to be dropped first, then the second, and so on. Unlike for variables, there is no need to reverse the drop order in this case, since Rust doesn’t (currently) allow self-references in a single value. So, Rust goes with the intuitive option.

### Inferior Mutability

> The Cell type in the standard library is an interesting example of safe interior mutability through invariants. It is not shareable across threads and never gives out a reference to the value contained in the Cell.

### Lifetime Variance

> 'static is a subtype of 'a because a 'static lives at least as long as any 'a and so is more useful. Or, more generally, if 'b: 'a ('b outlives 'a), then 'b is a subtypeof 'a. This is obviously not the formal definition, but it gets close enough to be of practical use.

### Alignment

1. Traditionally, computer hardware pointer accesses memory in the unit of `byte`, so value in memory should be stored in `byte aligned` manner, otherwise, it has to cost `2` read instead of `1` read to access a value.
2. Architecture has its `native alignment`, for x64, this is 64 bit. Therefore, store an `i64` with `8 byte alignment` means we need only `1` read to retrieve this value instead of `2` read
3. A __naturally aligned__ value is one __whose alignment matches its size__, which avoids poor performance.

### Layout

1. Rust compiler, by default, does not guarantee the order of fields in memory
2. Use `repr(C)` to get C-like alignment
3. Use `repr(n)` to get array-like alignment, array elements have no padding in between
4. Use `repr(packed)` to cancel padding in structure

> Tuple Represented like a struct with fields of the same type as the tuple values in the same order. 
> Array Represented as a contiguous sequence of the contained type with no padding between the elements. 
> Union Layout is chosen independently for each variant. Alignment is the maximum across all the variants. 
> Enumeration Same as union, but with one additional hidden shared field that stores the enum variant discriminant. The discriminant is the value the code uses to determine which of the enum variants a given value holds. The size of the discriminant field depends on the number of variants.

## Types

### Static Dispatch

1. `Static Dispatch`, when you use `impl Trait` or `T: impl Trait`, the compiler copies and pastes correspoding implementations for you. Note that, the methods you don't use won't get copied; you only pay for what you use.
2. `Static Dispatch` is efficient when the program is running, but it does increase compilation time because of larger amount of code to compile
3. `Non-Generic inner functions`, some parts of a function's logic is not related to type, so it can be extracted as an inner function. Doing so, this part of code don't have to be copied and pasted for multiple times, compiler will do some optimizations.

### Dynamic Dispatch

1. Some types are not sized, meaning it does not have known size at compile time, for example, slice
2. `?Sized` means maybe not size; `!Sized` means unknown size
3. `Fat pointer`, or `Wide pointer`, store not only the pointer to the generic type, but also the __additional information to find the right implementation of method__  of the type, for example, `Arc` and `Box`
4. `&dyn Trait`, means a fat pointer storing both `Vtable` and __pointer to the object__; For `Vtable` in Rust, refer to [doc of `std::task::RawWakeVTable`](https://doc.rust-lang.org/std/task/struct.RawWakerVTable.html)
5. Dynamic dispatch cuts compile time, but brings overhead in runtime to look up the function to call via `Vtable`

### Generic Trait

1. Two ways: `associate type` or `generic type parameter`
2. Use `associate type` to guarantee there is only one implementation for a type; `generic type parameter` can cover more than one type once, adding overhead to compiler to find which implementation a type should use when compling.
3. `Blanket implementations`, only the library which defines the trait can do something like this:

```Rust
impl MyTrait for T
where T: ...
{}
```
4. 
