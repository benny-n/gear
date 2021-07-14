# gear
**G**ame **E**ngine **@** **R**ust

Open source game engine implemented in Rust. (the name is still debatable..)

---
### Developement
In the first stages of developement, it should be decided which parts of the engine are to be implemented from scratch and which parts can be integrated from already existing crates.

Below are the most significant parts that form a game engine listed with some (definitely not all) of their components:

---
### 1. ECS
"Entity–component–system (ECS) is a software architectural pattern that is mostly used in video game development. ECS follows the principle of composition over inheritance, meaning that every entity is defined not by a "type" assigned to it, but by named sets of data, called "components", that are associated to it, which allows greater flexibility. In an ECS, processes called "systems" determine which entities have certain desired components, and act on all of those entities, using the information contained in the components. For example a physics system may query for entities having mass, velocity and position components, and then iterate over all of them and do physics calculations."
*-Wikipedia*

Items to focus on:
  * Entities
  * Components
  * Systems 
  
  ![image](https://user-images.githubusercontent.com/66024037/125699694-0c693777-86ef-4f26-8471-a13e88a03e36.png)
  
  ### relevant crates:
  - https://crates.io/crates/bevy_ecs 
  - https://crates.io/crates/hecs
  
  ### references:
  - https://docs.unity3d.com/Packages/com.unity.entities@0.14/manual/ecs_core.html
  - https://www.youtube.com/watch?v=2rW7ALyHaas&ab_channel=BoardToBitsGames
  
---

### 2. Math Engine

"A Math Engine is the veins of a Rendering Engine (item 3). Without it a 3D object can't rotate nor translate across the screen." 
*-Random game engine indie developer*

Items to focus on:
  * Vectors
  * Rotation matrices
  * Quaternions
  
  ![image](https://user-images.githubusercontent.com/66024037/125697297-8ce544ad-4930-4aa0-ab81-8281e90cf669.png)
  
  ### relevant crates:
  - https://crates.io/crates/cgmath 
  - https://crates.io/crates/glam

  ### references:
  - https://www.haroldserrano.com/math-engine-project/
  
---
### 3. Rendering Engine
"In a software application the rendering engine is the module that is reasonable for generating the graphical output. Basically the job of a rendering engine is to convert the applications internal model into a series of pixel brightness's that can be displayed by a monitor (or other graphical device e.g a printer). In a 3D game, the rendering engine might take a collection of 3D polygons as inputs (as well as camera and lighting data) and use that to generate 2D images to be outputted to the monitor." *-Quora*

Items to focus on:
  * Wrappers of OpenGL API
  * Shaders (a somewhat complicated topic that should probably have its own section)
  * And many, many more

  ![image](https://user-images.githubusercontent.com/66024037/125697719-c1117460-9041-476b-9a71-0371cc6f7682.png)
  
  ### relevant crates:
  - https://crates.io/crates/glium 
  - https://crates.io/crates/kiss3d

  ### references:
  - https://www.haroldserrano.com/articles/#openglsection (leads to a lot more links about OpenGL and other related topics).

---
### 4. Physics Engine
"A physics engine is computer software that provides an approximate simulation of certain physical systems, such as rigid body dynamics (including collision detection), soft body dynamics, and fluid dynamics, of use in the domains of computer graphics, video games and film (CGI). Their main uses are in video games (typically as middleware), in which case the simulations are in real-time." *-Wikipedia*

Items to focus on:
  * Forces and gravity 
  * Velocity and accelaration
  * Collision detection system
  
  ![image](https://user-images.githubusercontent.com/66024037/125698598-b2b62a0c-5f1f-449c-862b-df834796cd24.png)
  
  ### relevant crates:
  - https://crates.io/crates/nphysics3d
  - https://crates.io/crates/nphysics2d 
  - https://crates.io/crates/ncollide2d
  - https://crates.io/crates/ncollide3d

  ### references:
  - https://www.youtube.com/watch?v=-_IspRG548E&ab_channel=Winterdev
  - https://www.haroldserrano.com/blog/how-a-physics-engine-works-an-overview
  
---
---




### Other Useful Links & Refs

 - https://www.haroldserrano.com/blog/how-to-become-a-game-engine-developer

    Harold Serrano's blog about developing a game engine.
    most of the references in this document are taken from his blog, mainly because of how thorough and extensive his guides are.
    
 - https://doc.rust-lang.org/book

    An example-oriented guide book to learn the Rust language, from "Hello World" to the language's most complex features.
    
 - https://arewegameyet.rs/

    A guide to the Rust game development ecosystem.

---
### This README
This readme file is obviously incomplete, and is going to be updated constantly at the beginning of the project. Feel free to add useful references to the relevant sections and expand this document as you see fit.

---
