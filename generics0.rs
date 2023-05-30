// A generic in Rust is a placeholder for a type. 
// It can be used to write code that can work with any type, or with a specific set of types

/*  This function can be used with any type of slice, 
    such as &[i32], &[String], or &[bool].
*/
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for element in list {
        if element > largest {
            largest = element;
        }
    }
    return largest;
}

/*  Generics can also be used to define generic types. 
    A generic type is a type that can be used with any type. 
    For example, you could define a generic struct that stores a value of any type. The struct would look like this:
*/
struct Value<T> {
    value: T,
}
