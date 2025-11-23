**DYNAMIC DISPATCH AND V-TABLES**
A virtual function in C++ is a function of a base class that can be overridden by derived class.

This allows for the correct function to be called at runtime since the compiler often cannot know what the concrete class of some object is. 

The compiler creates a table of function pointers where each class in an inheritance structure has its set of function pointers. This way the compiler can dynamically dispatch the correct function during runtime by checking the concrete type of the object against the vtable for that type (or base class). Not sure if that's all right in terms of how I described the implementation but the general idea is correct. 
*might be cool to know where these things are stored*


