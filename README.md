# Matrical

## Overview
Matrical is a Rust library aimed to be a powerful and flexible way to manage and manipulate matrices. The library is currently under active development and is not yet ready for production use. However, it is open for contributions from anyone interested in its development.

## Intended Purposes
Simple. Worthy. Strategic.

## Roadmap
While Matrical is still in the early stages of development, the following is a rough roadmap of planned features and improvements:

Enhance Gear and Cog Functionality: This relationship allows for a high degree of flexibility and precision in managing matrix operations. By using Cogs to provide context and Gears to apply targeted operations, you can handle complex matrix operations in a more efficient and controlled manner.

Database integration: SurrealDb is an embedded database with multiparadigm support, including key-value, relational, document, and graph databases. Matrical will default to using SurrealDB for its database integration, but it should also provide support for other database systems. 

Automatic Parallelization: Matrical will automatically parallelize operations on matrices when possible using Rayon. This will allow for more efficient computation of large matrices. 

Thread safety: Matrical will be thread safe, allowing for concurrent access to matrices. Using Crossbeam, Matrical will provide support for concurrent reads and writes to matrices down to the element level. Most of the operations are already thread safe, but there are still a few that need to be worked on to ensure full thread safety.

Zero Copy: Matrical will use zero copy techniques to avoid unnecessary copying of data. This will allow for more efficient memory usage and faster computation. 

## Strategy and Factory Patterns
The Strategy pattern is a behavioral design pattern that enables an algorithm's behavior to be selected at runtime. In the context of Matrical, we use the Strategy pattern to define a family of algorithms, encapsulate each one, and make them interchangeable. This allows the algorithm to vary independently from clients that use it, in runtime!. For instance, different strategies can be applied to manipulate the matrix data based on the associated "Gear" or "Cog", change the current view of the matrix elements by changing the "Lens". 

The Factory pattern is a creational design pattern that provides an interface for creating objects in a superclass, but allows subclasses to alter the type of objects that will be created. In Matrical, we use the Factory pattern to create different types of "Gears" or "Cogs", each with their unique behaviors and operations.

## Contributing
Contributions to Matrical are welcome and appreciated. Whether you're fixing bugs, adding new features, improving performance, or enhancing documentation, your contributions can help make Matrical a powerful tool for working with matrices in Rust. 

Before contributing, please read the CONTRIBUTING.md file for guidelines on how to contribute to this project.

## License
Matrical is licensed under the MIT License.

## Disclaimer
Please note that Matrical is currently under active development and is not yet ready for production use. Use at your own risk. We appreciate your patience and contributions as we work to improve this library.


