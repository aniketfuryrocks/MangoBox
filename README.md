<h1 align="center">
  <br>
  <img src="https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022/blob/main/extension/images/mango.png" alt="Markdownify" width="200"></a>
  <br style="font-size:300%;">
   MangoBox
  <br>
</h1>

<h4 align="center">A set of utilities to make the lives of mango market users easier</h4>


<p align="center">
  <a href="#key-features">Inspiration</a> •
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#Technologies Used ">Technologies Used </a> •
  <a href="#license">License</a>
</p>


# Inspiration ☝

### MangoChat:
A chat room for mango market users based on this [backlog](https://trello.com/c/n2HgLkvt/102-%F0%9F%91%B9-trollbox) on the mango markets community trello board.
It helps the users of mango markets to connect with each other and more over provides a sand-boxed experience in the form of a browser extension to that the core user experience of mango markets is not disturbed.

### MangoTui:
A tui client which shows the prices of various cryptocurrencies in realtime  right from the user's own terminal.

# Key Features 🔑

- Allows the traders to communicate about their trade 

*WIP*

- Authenticate with Mango Account
- Access rooms from the UI
- Spam Protection
- Health Bot

# Technologies Used 🤵
- [Mango Markets](https://github.com/blockworks-foundation/mango-v3)
- [Rust](https://www.rust-lang.org/)
- [Browser Extension](https://developer.chrome.com/docs/extensions/)
- [tui-rs](https://github.com/fdehau/tui-rs)

# How To Use ⚙

### MangoChat:
```bash
# Clone this repository
$ git clone https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022.git

# Go into the repository
$ cd mlh-orientation-hackathon-fall-2022

# run chat-server
$ cargo run

```
### MangoTui:
```bash
# Clone this repository
$ git clone https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022.git

# Go into the repository
$ cd mlh-orientation-hackathon-fall-2022

# Switch to the tui branch
$ git checkout tui

# run tui client
$ cargo run --bin mango-tui

```


## Meet Our Team 22.FAL.5 🙋🏻

<a href="https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=aniketfuryrocks/mlh-orientation-hackathon-fall-2022" />
</a>

## License & Copyright

Licensed under the **[MIT LICENSE](LICENSE)**

