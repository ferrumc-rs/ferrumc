import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.server.Bootstrap;
import net.minecraft.SharedConstants;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class MappingExtractor {
    public static void main(String[] args) {
        try {
            if (args.length < 1) {
                System.exit(1);
            }
            SharedConstants.tryDetectVersion();
            Bootstrap.bootStrap();
            File outputFile = new File(args[0]);

            try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputFile))) {
                writer.write("{\n");

                // --- 1. Placement: Item -> Default Block State ---
                writer.write("\"placement\": {\n");
                writePlacementMap(writer);
                writer.write("\n},\n");

                // --- 2. Lookup: Block State -> Item ---
                writer.write("\"lookup\": {\n");
                writeLookupMap(writer);
                writer.write("\n}\n");

                writer.write("}\n");
            }
        } catch (Throwable e) {
            e.printStackTrace();
            System.exit(1);
        }
    }

    private static void writePlacementMap(BufferedWriter writer) throws Exception {
        List<String> lines = new ArrayList<>();
        for (Item item : BuiltInRegistries.ITEM) {
            if (item instanceof BlockItem) {
                BlockItem blockItem = (BlockItem) item;
                int itemId = BuiltInRegistries.ITEM.getId(item);
                int blockStateId = Block.getId(blockItem.getBlock().defaultBlockState());
                lines.add(String.format("\"%d\": %d", itemId, blockStateId));
            }
        }
        writeLines(writer, lines);
    }

    private static void writeLookupMap(BufferedWriter writer) throws Exception {
        List<String> lines = new ArrayList<>();
        for (Block block : BuiltInRegistries.BLOCK) {
            Item item = block.asItem();
            // Skip blocks that don't have an item form (like Nether Portal or Fire)
            if (item == Items.AIR)
                continue;

            int itemId = BuiltInRegistries.ITEM.getId(item);

            // Map EVERY valid state of this block to the item
            for (BlockState state : block.getStateDefinition().getPossibleStates()) {
                int stateId = Block.getId(state);
                lines.add(String.format("\"%d\": %d", stateId, itemId));
            }
        }
        writeLines(writer, lines);
    }

    private static void writeLines(BufferedWriter writer, List<String> lines) throws Exception {
        // Sort numerically by Key (ID) for clean output
        Collections.sort(lines, (a, b) -> {
            int idA = Integer.parseInt(a.split("\"")[1]);
            int idB = Integer.parseInt(b.split("\"")[1]);
            return Integer.compare(idA, idB);
        });

        for (int i = 0; i < lines.size(); i++) {
            writer.write(lines.get(i));
            if (i < lines.size() - 1)
                writer.write(",\n");
        }
    }
}