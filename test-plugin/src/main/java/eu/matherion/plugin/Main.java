package eu.matherion.plugin;

import org.bukkit.event.Listener;
import org.bukkit.plugin.java.JavaPlugin;

/**
 * Just a test-plugin that's automatically included in the dev server.
 */
public class Main extends JavaPlugin implements Listener {

    @Override
    public void onEnable() {
        super.onEnable();
        this.getServer().getPluginManager().registerEvents(this, this);
    }

   /* @EventHandler
    public void onChat(AsyncPlayerChatEvent e) throws InterruptedException {
        for (Sound sound : Sound.values()) {
            System.out.println(sound);
            e.getPlayer().playSound(e.getPlayer().getLocation(), sound,1F, 1F);
            Thread.sleep(1000);
        }
    }*/

}
