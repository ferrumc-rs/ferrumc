import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.Bootstrap;
import net.minecraft.SharedConstants;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class BlockInfoDumper {
      public static void main(String[] args) {
            try {
                  if (args.length < 1) {
                        System.err.println("Usage: java BlockInfoDumper <output_file>");
                        System.exit(1);
                  }

                  SharedConstants.tryDetectVersion();
                  Bootstrap.bootStrap();
                  File outputFile = new File(args[0]);

                  List<String> outputLines = new ArrayList<>();

                  for (Block block : BuiltInRegistries.BLOCK) {
                        ResourceLocation key = BuiltInRegistries.BLOCK.getKey(block);
                        BlockState defaultState = block.defaultBlockState();

                        float hardness = defaultState.getDestroySpeed(null, null);
                        float resistance = block.getExplosionResistance();
                        float friction = block.getFriction();
                        float speedFactor = block.getSpeedFactor();
                        float jumpFactor = block.getJumpFactor();

                        int lightEmission = defaultState.getLightEmission();
                        boolean isAir = defaultState.isAir();

                        // --- FIX: Safe Collision Check ---
                        boolean isSolid = false;
                        try {
                              // blocksMotion() internally calls getCollisionShape().
                              // Complex blocks (Pistons, Redstone) might crash here if the world is null.
                              isSolid = defaultState.blocksMotion();
                        } catch (Exception e) {
                              // If it crashes, it's likely a complex technical block.
                              // Defaulting to false (not solid) is the safest fallback.
                              isSolid = false;
                        }
                        // ---------------------------------

                        String line = String.format(
                                    "\"%s\": {" +
                                                "\"hardness\": %s," +
                                                "\"resistance\": %s," +
                                                "\"friction\": %s," +
                                                "\"speed_factor\": %s," +
                                                "\"jump_factor\": %s," +
                                                "\"light_emission\": %d," +
                                                "\"is_air\": %b," +
                                                "\"is_solid\": %b" +
                                                "}",
                                    key.toString(),
                                    hardness, resistance, friction, speedFactor, jumpFactor,
                                    lightEmission, isAir, isSolid);
                        outputLines.add(line);
                  }

                  Collections.sort(outputLines);

                  try (BufferedWriter writer = new BufferedWriter(new FileWriter(outputFile))) {
                        writer.write("{\n");
                        for (int i = 0; i < outputLines.size(); i++) {
                              writer.write(outputLines.get(i));
                              if (i < outputLines.size() - 1)
                                    writer.write(",\n");
                              else
                                    writer.write("\n");
                        }
                        writer.write("}\n");
                  }

            } catch (Throwable e) {
                  e.printStackTrace();
                  System.exit(1);
            }
      }
}