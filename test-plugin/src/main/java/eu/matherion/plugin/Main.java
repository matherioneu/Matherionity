package eu.matherion.plugin;

import eu.matherion.api.servers.data.ServerStateUpdate;
import eu.matherion.api.servers.pubsub.subscriber.ServerStateUpdateSubscriber;
import org.bukkit.event.Listener;
import org.bukkit.plugin.java.JavaPlugin;

/**
 * Just a test-plugin that's automatically included in the dev server.
 */
public class Main extends JavaPlugin implements Listener {

    @Override
    public void onEnable() {
        System.out.println("MEOWMEOW");
        super.onEnable();
        this.getServer().getPluginManager().registerEvents(this, this);
        this.getServer().getPubSub().addServerStateUpdateSubscriber(serverStateUpdate -> {
            getLogger().info("TEST FROM PLUGIN");
            System.out.println(serverStateUpdate);
        });
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
