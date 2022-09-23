<h1 align="center">
  <br>
  <img src="https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022/blob/main/extension/images/mango.png" alt="Markdownify" width="200"></a>
  <br style="font-size:300%;">
   MangoBox
  <br>
</h1>

<h4 align="center">A set of utilities to make the lives of Mango Market users easier</h4>


<p align="center">
  <a href="#inspiration-">Inspiration</a> •
  <a href="#key-features-">Key Features</a> •
  <a href="#how-to-use-">How To Use</a> •
  <a href="#technologies-used-">Technologies Used </a> •
  <a href="#license--copyright">License</a> •
  <a href="https://www.youtube.com/watch?v=I1sbMoO2CKQ">Demo</a>
</p>

# Inspiration ☝

### MangoChat:
A chat room for mango market users based on this [backlog](https://trello.com/c/n2HgLkvt/102-%F0%9F%91%B9-trollbox) on the mango markets community trello board with whom we will be working during the fellowship. It helps the users of mango markets to connect with each other and more over provides a sand-boxed experience in the form of a browser extension to that the core user experience of mango markets is not disturbed.

### MangoTui:
A tui client which shows the prices of various cryptocurrencies in realtime  right from the user's own terminal.

# Key Features 🔑

### MangoChat
- Allows traders to communicate about their trade.
- It has minimal interface and can extend to a web page but main focus is to provide good chat functionality with less overhead.

### MangoTui
- Allows traders to view crypto prices in realtime right from their terminal

# Technologies Used 🤵
- [Mango Markets](https://github.com/blockworks-foundation/mango-v3)
- [Rust](https://www.rust-lang.org/) 
- [Browser Extension](https://developer.chrome.com/docs/extensions/)
- [tui-rs](https://github.com/fdehau/tui-rs)

# How To Use ⚙

> Prerequisites 
> - [rust](https://www.rust-lang.org/tools/install)
> - [Manifest v3 Supported Browser](https://extensionworkshop.com/documentation/develop/manifest-v3-migration-guide/)

### MangoChat:
```bash
# Clone this repository
$ git clone https://github.com/aniketfuryrocks/MangoBox.git

# Go into the repository
$ cd MangoBox

- Add the Extension in your browser

- Go to More Tools and then choose Extensions from the browser menu.
- Enable the Developer mode here.
- click the Load unpacked button then select the Extension folder

# run chat-server
$ make server

```

### MangoTui:
```bash
# Clone this repository
$ git clone https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022.git

# Go into the repository
$ cd mlh-orientation-hackathon-fall-2022

# run tui client
$ make tui

```

# Future Goal ⌛

### MangoChat:

- Authenticate with Mango Account
- Access rooms from the UI
- Spam Protection
- Health Bot

### MangoTui:

- Setting user alerts for Cryto rates.
- Add more Stats like Borrow, Deposite rates from Mango Market platform



# Meet Our Team 22.FAL.5 🙋🏻

<a href="https://github.com/aniketfuryrocks/mlh-orientation-hackathon-fall-2022/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=aniketfuryrocks/mlh-orientation-hackathon-fall-2022" />
</a>

# License & Copyright

Licensed under the **[MIT LICENSE](LICENSE)**

