# Different Way of handling errors

### unwrap

```
unwrap_or(0)
unwrap() // panics
```

### Option

```
<T> -> Value
None
```

### Result

```
<T> -> Value
<E> -> Error
```

### Convert Option to Result

```
ok_or()
```

### Convert Result to Option

```
ok()
```

### To Map Error

```
map(|e| e.unwrap_err())
```

### Propagation

```
? -> to Propagate the error up the stack
```
