# Verbose_Bird

------

# FR:

## Utilisation:

Le *pack de loggers* est composé de deux "modes":

- Le mode procédural.
- Le mode programmation orientée objet.

Si votre application n'est pas orientée objet, vous pouvez simplement utiliser les outils adaptés au mode procédural, comme ceci:

```Rust

#[macro_use]
extern crate verbose_bird;


fn main()
{
    info!("say something");
}

```

Pour le pack de loggers dédié à la programmation orientée objet, veuillez importer ce projet sur votre machine et utiliser la commande `cargo doc`.

# EN:

## Use:

The *loggers pack* is made from two "modes":

- Procedural Programming.
- Object-oriented programming.

If your programming is not Object-oriented, you can simply use the tools from the Procedural Programming, as follows:

```Rust

#[macro_use]
extern crate verbose_bird;


fn main()
{
    info!("say something");
}

```

For the object-oriented programming, please clone this repo and run `cargo doc` command.