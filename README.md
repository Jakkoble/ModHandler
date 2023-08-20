# ModHandler

Isn't it annoying to switch the Minecraft Mods directory content between different game versions/modpack sessions? With this simple tool you will experience a **new, faster Way** to switch the Content of your Minecraft mods folder.
</br> </br>

![terminal](https://i.imgur.com/0mUuBeq.png)

## Setup

1. Just download the **[latest Release File](https://github.com/jakkoble/ModHandler/releases/latest)** for your operating system
2. Move the file where ever you want on your computer (e. g. on you desktop)
3. Execute the binary

_Yeah, it's that simple!_
</br>

## Create new Profiles

The **ModHandler** works with **Profiles**. These are just directories inside the `profiles` _(created on first execution)_ directory. Inside of these Profiles you can put what ever Mod/Config/File you'd like.
Therefore, if you want to create a new Profile you just have to create a new directory inside the `profiles` directory and fill it with the content you want.
</br>
</br>

## Specify different .minecraft directory

The default Path of the `.minecraft` directory is the `%appdata%/.minecraft` path. (Default for Windows installation). </br> </br>
If you are using a differnt OS like Linux or you've installed the `.minecraft` directory somewhere else, you can specify this Path by adding a `path.txt` file
in the same directory as the `mod_handler` binary and put the full path to the `.minecraft` directory inside the `path.txt` file.
</br></br>
Example Content of path.txt:</br>

```
/home/jakob/.minecraft
```

</br> </br>
If you need further help, feel free to **[open a new Issue](https://github.com/Jakkoble/ModHandler/issues/new)**. In case you like my project, please give this Repository a ⭐.
