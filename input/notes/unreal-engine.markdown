---
title = "Unreal Engine"
published = "December 17, 2016"
comments = false
---

Unreal Engine is an industry grade, robust game engine. While previous engines offered UnrealScript for implementing new behavior, Unreal Engine 4 uses C++ exclusively along with visual programming "Blueprints" that are generate C++.

<toc/>

# Editor

Pressing <kbd>F</kbd> in the camera pane will move the camera to focus onto the selected object.

Pressing and holding <kbd>ALT</kbd> while dragging left-click will orbit the focused object, while dragging right-click will zoom in and out of the object.

Geometry brushes have built-in behavior for collision, tessellation, and material tiling. They are used to "block out" an environment and then they are converted to static meshes.

A brush face can be selected by holding <kbd>CTRL</kbd> + <kbd>SHIFT</kbd> + left-click on a face. All faces can then be selected by going to the _Geometry_ drop-down and selecting _Select all adjacent faces_.

Objects such as brushes can be duplicated by copy-pasting or by holding <kbd>ALT</kbd> while transforming the object.

Physics simulation can be enabled for an object by going to its _Physics_ section of the _Details_ panel and ticking the _Simulate Physics_ checkbox.

The physics of an object can be constrained to specific planes, such as the YZ-plane, by using the _Constraints_ field in the _Physics_ section of the _Details_ panel.

A _trigger volume_ is one that emits an event if an actor collides with it.

