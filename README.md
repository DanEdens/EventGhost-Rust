# EventGhost


This is a test version of EventGhost. It is a work in progress.
This test version runs on Python 3.5 x64


## build requirements

* visual c >= 14.0 \*
* winows sdk >= 8.1
* cx_Freeze >= 5.1.1
* requests >= 2.19.1
* agithub >= 2.1
* pycurl >= 7.43.0.2
* qrcode >= 6.0
* tornado >= 5.1
* psutil >= 5.4.7
* websocket-client-py3 >= 0.15.0
* CommonMark >= 0.7.5
* comtypes >= 1.1.7
* future >= 0.16.0
* Pillow >= 5.2.0
* PyCrypto >= 2.6.1
* Sphinx >= 1.8.0b1
* wxPython >= 4.0.3
* pywin32 >= 223
* setuptools >= 40.2

\* Visual C is also comes with Visual Studio. You will need Visual Studio >= 2015


## build command

    python setup.py build_exe

Lessons Learned
--------------

Async Implementation Considerations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
When porting EventGhost plugins to Rust, we learned that maintaining the original design philosophy is crucial:

1. **Simplicity Over Complexity**: 
   - EventGhost's original design favors straightforward, synchronous event handling
   - Avoid over-engineering with complex async patterns unless absolutely necessary
   - Keep plugin interfaces simple and predictable

2. **Plugin Design Principles**:
   - Maintain simple start/stop lifecycle methods
   - Use direct event handling rather than complex async event chains
   - Focus on immediate event processing rather than queuing/pooling

3. **Resource Management**:
   - Prefer simple, direct resource handling over complex pooling
   - Use straightforward cleanup methods
   - Maintain EventGhost's original resource lifecycle patterns

4. **When to Use Async**:
   - Only introduce async when dealing with inherently async operations (e.g., network I/O)
   - Keep async boundaries at the edges of the system
   - Don't force async patterns where synchronous code would be clearer

Example of Simplified Plugin Structure:
```rust
pub trait Plugin {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
    fn handle_event(&mut self, event: &Event) -> Result<(), Error>;
}
```

This approach better matches EventGhost's original design while still allowing for Rust's safety and performance benefits.
