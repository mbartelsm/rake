use winit::keyboard;

mod ffi;
mod utils;

// TODO:
//  - GUI in C#
//      - Menu with Undo, Redo, and Close.
//      - Canvas for bitmap drawing.
//  - C# Event handling and forwarding to Rust
//      - Keyboard and mouse events over canvas
//  - Rust event handling
//      - Events to Actions
//      - Actions to output
//      - Undo/Redo buffer
//  - Rust GPU processing and buffering
//      - Perspective view
//      - Draw a grid
//      - Draw a 3D cursor
//      - Draw quads with the given vertices
//  - C# drawing from buffer
//      - Double buffering to canvas

enum Action {
    None,
    GlobalAction(GlobalAction),
    EditorAction(EditorAction),
}

enum GlobalAction {
    Undo,
    Redo,
    CloseProgram,
}

enum EditorAction {
    MoveCursor(),
    AddVertex(),
}

enum Event {
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
}

enum MouseEvent {
    CursorEntered { entry: Point2 },
    CursorExited { exit: Point2 },
    CursorMoved { delta: Point2 },
    CursorWheel { delta: i32 },
    ButtonPressed(MouseButton),
    ButtonReleased(MouseButton),
}

enum KeyboardEvent {
    KeyPressed(Key),
    KeyReleased(Key),
}

struct Key {
    physical_key: keyboard::PhysicalKey,
    logical_key: keyboard::Key,
}

enum MouseButton {
    Left,
    Right,
    Middle,
    Mouse4,
    Mouse5,
}

type Point2 = ultraviolet::int::IVec2;
