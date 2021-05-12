package eu.matherion.plugin;

import com.fasterxml.jackson.databind.ObjectMapper;
import eu.matherion.api.servers.ServerState;
import eu.matherion.api.StateType;

import java.io.IOException;

class Scratch {
    public static void main(String[] args) throws IOException {
        ObjectMapper mapper = new ObjectMapper();
        System.out.println(mapper.canSerialize(ServerState.class));
        String json = mapper.writeValueAsString(new ServerState(2, "TheBridge", null, StateType.WAITING, false, 0, 10));
        System.out.println(json);
        ServerState rejsoned = mapper.readValue(json, ServerState.class);
        System.out.println(rejsoned);
    }
}