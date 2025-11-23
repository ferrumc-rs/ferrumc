import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.MobCategory;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.Bootstrap;
import net.minecraft.SharedConstants;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class EntityExtractor {
      public static void main(String[] args) {
            try {
                  if (args.length < 1) {
                        System.err.println("Usage: java EntityExtractor <output_file>");
                        System.exit(1);
                  }
                  SharedConstants.tryDetectVersion();
                  Bootstrap.bootStrap();
                  File outputFile = new File(args[0]);

                  List<String> outputLines = new ArrayList<>();

                  for (EntityType<?> type : BuiltInRegistries.ENTITY_TYPE) {
                        ResourceLocation key = BuiltInRegistries.ENTITY_TYPE.getKey(type);
                        int id = BuiltInRegistries.ENTITY_TYPE.getId(type);

                        float width = type.getWidth();
                        float height = type.getHeight();
                        // eyeHeight logic is complex in vanilla (depends on pose), skipping for now or
                        // using fallback

                        boolean summonable = type.canSummon();

                        boolean fireImmune = type.fireImmune();
                        boolean canSpawnFar = type.canSpawnFarFromPlayer();

                        MobCategory category = type.getCategory();
                        String catName = category.name(); // "MONSTER", "MISC"

                        String line = String.format(
                                    "\"%s\": {" +
                                                "\"id\": %d, \"width\": %f, \"height\": %f, " +
                                                "\"summonable\": %b, \"fire_immune\": %b, \"can_spawn_far\": %b, " +
                                                "\"category\": \"%s\"" +
                                                "}",
                                    key.toString(), id, width, height, summonable, fireImmune, canSpawnFar, catName);
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