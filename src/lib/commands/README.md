# **FerrumC Commands**

Layer: 4 (Infrastructure)  
Dependencies: ferrumc-core, ferrumc-ecs

## **Purpose**

This crate provides the command parsing engine and dispatching infrastructure for the server. It implements a command tree structure similar to Mojang's "Brigadier" library.  
It is responsible for:

1. **Defining Syntax:** Creating a graph of Literals and Arguments.  
2. **Parsing:** Converting raw string inputs ("/tp @a 0 100 0") into structured data.  
3. **Dispatching:** Invoking the appropriate handler when a command is successfully parsed.

## **Architectural Boundaries**

### **What belongs here?**

* The CommandDispatcher struct.  
* The CommandNode, Literal, and Argument definitions.  
* Parsing logic (StringReader, Cursor handling).  
* The CommandContext struct (passed to executors).  
* The CommandSender trait/enum.

### **What does NOT belong here?**

* **Command Implementations:** The actual logic for commands like /gamemode, /tp, or /stop does **not** live here. Those belong in their respective Game Modules (e.g., ferrumc-plugin-management, ferrumc-plugin-movement).  
* **Game Logic:** This crate should not know about specific components like Hunger or ChunkStorage.

## **Usage Guide**

### **Registering a Command**

Commands are registered by plugins during app initialization.  
```rust
pub fn register(dispatcher: \&mut CommandDispatcher) {  
    // Register: /test \<number\>  
    dispatcher.root.add\_child(  
        Literal::new("test")  
            .then(  
                Argument::new("value", IntegerArgument)  
                    .executes(handle\_test\_command)  
            )  
    );  
}

fn handle\_test\_command(ctx: CommandContext) {  
    let value: i32 \= ctx.get\_argument("value");  
    ctx.sender.send\_message(format\!("You entered: {}", value));  
}
```

### **The Command Flow**

1. **Input:** ferrumc-net or ferrumc-management receives a command string.  
2. **Event:** An ECS Event (e.g., ConsoleCommandEvent) is fired.  
3. **System:** The Command Dispatcher system reads the event.  
4. **Parse:** The string is traversed against the Command Graph.  
5. **Execute:** If valid, the attached function pointer or closure is executed.  
6. **Logic:** The closure typically fires a specific gameplay event (e.g., ChangeGameModeEvent) into the ECS.

