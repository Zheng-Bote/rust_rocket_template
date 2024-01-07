<div id="top"></div>
<br />
<div align="center">

# Rust Webserver with Rocket

Self-contained System.

Basic pre-configured Rust Webserver with Rocket, CORS, CSRF, Fileserver for Tera-Templates and static assets.

</div>

## Status

![ops](https://img.shields.io/badge/Status-usable-green)

![dev](https://img.shields.io/badge/Info-limited_functionality-yellow)
**Still under development**

## Features

- [x] Rust programming language

- [x] Rocket webframework

- [x] Self-contained System with Web-based Userinterface

- [x] CORS

- [x] CSRF (Argon2)

- [x] Rocket Fileserver with automatic static asset support

- [x] template-based UI with Tera, JS and CSS support

- [x] responsive web design with CSS3 (mobile first)

- [x] i18n UI frontend support with JSON (Browser auto-detection and switchable)

- [x] dark-/light-mode switchable

- [x] remote shutdown (shutdown API)

- [x] logfiles with typical log-levels

- [x] configured error-handling (400, 404, 422, 500)

- [ ] display current application-log

- [ ] configure CSRF fairing for put/delete/post

- [ ] tbd.

## HISTORY:

> | Version | Date       | Developer | Comments              |
> | ------- | ---------- | --------- | --------------------- |
> | 0.1.0   | 2023-12-16 | RZheng    | created               |
> | 0.1.1   | 2024-01-07 | RZheng    | added: user/loginform |

## Authors

- [@Zheng-Bote](https://www.github.com/zheng-bote)

## License

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

## API Reference

### Landing Page

loads the landing page with some explanations

```http
  GET /
```

**Returns:**
HTML page (template)

#### user/login

load login-form

```http
  GET /user/login
```

| Parameter | Type   | Description      |
| :-------- | :----- | :--------------- |
| `none`    | `html` | loads login-form |

**Returns:**
HTML page (template)

#### assets/static content

Fileserver provides static assets

```http
  GET /assets/<index.html>
```

| Parameter | Type        | Description        |
| :-------- | :---------- | :----------------- |
| `none`    | `mime-type` | loads static asset |

**Returns:**
static asset, auto-detection of mime-type

#### shutdown Rocket webserver

gracefull shutdown the rocket webserver
**not yet secured, accessable for everyone**

```http
  GET /shutdown
```

| Parameter | Type  | Description     |
| :-------- | :---- | :-------------- |
| `none`    | `GET` | system shutdown |

**Returns:**
string

## Setup

### Environment Variables

To run this project, you will need to add the following environment variables to your `.env` file (see example: env_example)

`DATABASE_URL="postgres://username:password@dbhost:port/database"`

For production, you need to configure Rocket secret_key or disable it in your `Rocket.toml`

### Database

not implemented within this template

### Logfile

stored in `<appfolder>/logs/application.log`

**_no log rotation implemented_**

default logging mode: normal

### run

`cargo run`

### build release

`cargo build --release`

## Security

the following CORS are defined (in main.rs):

```
response.set*header(Header::new("Access-Control-Allow-Origin", "*"));
response.set*header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS", ));
response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

response.set_header(Header::new("Strict-Transport-Security", "max-age=63072000"));
```

**please be aware of "Strict-Transport-Security"**

The CSRF token is build with Argon2

## Screenshots

**Startpage**

Example with english language (Example comes with EN and DE).

Header area with left hand brand icon and brand title

Header area with right hand navigation icons (toggle Language, toggle mode, login)

  <img src="https://raw.githubusercontent.com/Zheng-Bote/rust_rocket_template/main/docs/template_01_en.png"  width="100%" height="auto" />

**Startpage**

Example with german language (Example comes with EN and DE).

  <img src="https://raw.githubusercontent.com/Zheng-Bote/rust_rocket_template/main/docs/template_01_de.png"  width="100%" height="auto" />

**dark mode / light mode**

  <img src="https://raw.githubusercontent.com/Zheng-Bote/rust_rocket_template/main/docs/template_03_darkmode.png"  width="100%" height="auto" />

**user/loginform**

  <img src="https://raw.githubusercontent.com/Zheng-Bote/rust_rocket_template/main/docs/template_04_loginform.png"  width="100%" height="auto" />

**Fileserver/static assets**

  <img src="https://raw.githubusercontent.com/Zheng-Bote/rust_rocket_template/main/docs/assets_01_index.png"  width="100%" height="auto" />

### the end

:vulcan_salute:

<p align="right">(<a href="#top">back to top</a>)</p>
