# Matherionity

The truly batteries-included Paper/Tuinity fork.

## The new API

While the old, legacy Bukkit API is still accessible and will continue to be updated, 
when making new plugins, you can choose to use the new API. The new API is not fully
complete though, as Matherionity is in early development. Also some parts of the API
are subject to change, however that may only happen in major releases.

## Features
- Redis
- Database Connection
- New Plugin Manager, inspired by Velocity (TBD)

## Requirements
- Java 11+ 
- MySQL
- Redis

### Development Requirements:
- Maven
- Git
- Bash
- GNU Patch

## Getting started

Assuming you have a Matherionity jar, create a new folder with it inside. Open your terminal 
and enter:

```shell
$ java -jar matherionity.jar
```

This will start the Matherionity server.

## Development

Matherionity uses patches. To learn more about them, see [Understanding Patches](https://github.com/PaperMC/Paper/blob/master/CONTRIBUTING.md#understanding-patches).

In the folder root, you will have a `tuinity` shell script which
is useful for development.

### Compiling the JAR

If you just want to compile the jar, use:

```shell
$ ./tuinity jar
```

Note: In Windows, please use Git Bash.

### Modifying the code

If you want to modify the code, use:

```shell
$ ./tuinity apply
```

This command will prepare everything for you and apply all patches. Once it's finished,
you will see two folders - Tuinity-Server & Tuinity-API.

### Rebuilding patches

To rebuild all patches, first commit in the Tuinity-Server and/or Tuinity-API repository.
Then, use:

```shell
$ ./tuinity rebuild
```

This will rebuild all patches.

## The API

### Redis

The default RedisManager implementation (`eu.matherion.server.redis.RedisManagerImpl`) uses the Redisson library.

You can access the RedisManager via `Server#getRedisManager()`.

Example:
```java
class YourPlugin {
    public void yourFunction() {
       // ...
       RedisManager redisManager = Bukkit.getServer().getRedisManager();
    }
}
```

### Available Functions

`eu.matherion.api.redis.RedisManager` is an interface. If you want to use a different library for Redis (Jedis, for example), you can create your own RedisManager
implementation (TBD - for now it cannot be changed).

#### T getClient();

Gets the Client. The basic RedisManager implementation uses Redisson &mdash;
See the [Redisson Docs](https://github.com/redisson/redisson/wiki/Table-of-Content) for more info.

#### Map<String, ServerState> getServers();

Gets all servers from Redis.

#### void removeServer(String server);

Removes the ServerState of a certain server.

#### Future<Map<String, ServerState>> getServersAsync();

This function is not implemented yet. If you try to use it, it will throw a `java.lang.UnsupportedOperationException`.

#### void setServerState(String server, ServerState serverState);

Sets the `ServerState` of a certain server. If you want to set a state of the current server,
use `Server#setServerState(ServerState)` instead.

#### Future<Void> setServerStateAsync(String server, ServerState serverState) throws ExecutionException, InterruptedException;

This function is not implemented yet. If you try to use it, it will throw a `java.lang.UnsupportedOperationException`.

## Database

Matherionity uses ORMLite for all database actions. In order to use the database, you will have to create a model (essentially just a POJO with few annotations). 

For example:

```java
package eu.matherion.plugin.db;

import com.j256.ormlite.field.DatabaseField;
import com.j256.ormlite.table.DatabaseTable;

@DatabaseTable
class User {

    @DatabaseField(id = true)
    private Integer id;

    @DatabaseField(unique = true)
    private String username;

    @DatabaseField
    private String password;
    
    public Integer getId() {
        return id;
    }
    
    // ...
}
```

In your Main class, register the model like this:

```java
package eu.matherion.plugin;

import eu.matherion.plugin.db.User;
import org.bukkit.plugin.JavaPlugin;
import java.util.List;

class Main extends JavaPlugin {

    @Override
    public List<Class> provideDatabaseEntities() {
        return List.of(User.class);
    }

}
```

And that's it. Now, how the heck do I make queries? For that, you can access the DatabaseManager through `Server#getDatabaseManager()`.
The DatabaseManager has a `getDao(Class)` function. Use it to get a DAO (Data Access Object) class for your 
model (in this case User).

```java
package eu.matherion.plugin;

// ...

import com.j256.ormlite.dao.Dao;

class Main extends JavaPlugin {

    // ...

    @Override
    public void onEnable() {
        // Integer is the ID type, User is the model
        Dao<User, Integer> userDao = Bukkit.getServer().getDatabaseManager().getDao(User.class);
    
        // Creating users:
        User newUser = userDao.create(new User(/* your logic to create the User class */));
        
        // Updaing users:
        User existingUser = /* your logic to fetch the user */;
        existingUser.username = "different username";
        userDao.update(existingUser);
    
        // Fetching users:
        userDao.queryForEq("username", "username"); // get by row, value
        userDao.queryForAll(); // get all
        userDao.queryForId(1); // get by id
        userDao.queryForSameId(/* another User */); // get by the same id as another User class instance
    }

}
```

For more info, please refer to the ORMLite [Documentation](https://ormlite.com/javadoc/ormlite-core/doc-files/ormlite.html#License) or [JavaDocs](https://ormlite.com/javadoc).

If you however need to use the raw connection, you can get a `eu.matherion.api.database DatabaseConnection` via
`DatabaseManager#getConnection()`. DatabaseConnection is a small interface providing DatabaseManager with Java's Connection.
The default implementation (eu.matherion.server.database.impl.HikariConnection) uses HikariCP, but if you want to create the connection differently or use a different SQL database, you can create your own implementation (TBD - for now it's only Hikari&MySQL).
