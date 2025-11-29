# **FerrumC Registry**

**Layer:** 2 (Engine State)  
**Dependencies:** ferrumc-core

## **Purpose**

This crate provides **Zero-Cost, Compile-Time access** to static Minecraft game data.  
Instead of parsing JSON files at runtime (which is slow and error-prone), this crate contains massive Rust files full of const definitions and match statements. This allows the compiler to optimize lookups into instant jump tables.  
**It answers questions like:**

* "What is the Protocol ID for minecraft:stone?"  
* "What is the hardness of minecraft:obsidian?"  
* "What block does a diamond\_pickaxe break?"

## **Auto-Generated Code**

**99% of the code in this crate is AUTO-GENERATED.**  
Do **NOT** edit files in src/generated/ manually. Your changes will be overwritten the next time the data generator runs.

### **How to Update Data (New Minecraft Version)**

If you need to update the registry for a new Minecraft version (e.g., 1.21 \-\> 1.22):

1. Open tools/data-gen/src/setup/mod.rs.  
2. Update the MC\_VERSION constant.  
3. Run the generator tool from the project root:  
   cargo run \-p ferrumc-data-gen

4. Commit the changes to src/generated/.

## **Usage Guide**

### **1\. Block Lookups**

Access block properties (Hardness, Resistance, State IDs) using the blocks module.  
```rust
use ferrumc\_registry::blocks;

// Lookup by Name (O(1))  
let stone \= blocks::get\_block\_by\_name("stone").unwrap();  
println\!("Hardness: {}", stone.hardness); // 1.5

// Lookup by State ID (O(1))  
let block \= blocks::get\_block\_by\_id(1).unwrap();  
assert\_eq\!(block.name, "minecraft:stone");
```
### **2\. Item Lookups**

Access item properties (Stack Size, Durability) using the items module.  
```rust
use ferrumc\_registry::items;

// Lookup by Name  
let pickaxe \= items::get\_item\_by\_name("diamond\_pickaxe").unwrap();  
println\!("Max Durability: {}", pickaxe.max\_damage);

// Lookup by ID  
let item \= items::get\_item\_by\_id(741).unwrap();
```

### **3\. Mappings (Block \<-\> Item)**

Convert between Block States (World) and Item IDs (Inventory).  
```rust
use ferrumc\_registry::mappings;

// Placing a block: Item ID \-\> Block State ID  
let block\_id \= mappings::get\_block\_id\_from\_item\_id(item.protocol\_id);

// Picking a block: Block State ID \-\> Item ID  
let item\_id \= mappings::get\_item\_id\_from\_block\_id(block\_state\_id);
```