﻿# Yew Storybook clone

### Introduction

Welcome to the Yew StorybookClone, a powerful visual testing suite inspired by Storybook for React. Designed specifically for Yew, a modern Rust framework for building web applications, our StorybookClone empowers developers to streamline the development and testing of Yew components.

The Yew StorybookClone provides a dedicated environment for visually testing Yew components in isolation from the main project.

**Your Main Yew project comes as a bare bones preset, along with the Storybook test suite.**
**TailwindCSS has been setup in both the Yew app aswell as in Storybook. Consult Trunks website, "https://trunkrs.dev/assets/", to implement a custom styling setup.**



### Prerequisites

- cargo generate
- trunkrs

### Installation

- cargo generate:

  - _cargo install cargo-generate_

- trunkrs:

  - _cargo install --locked trunk_

- Yew Storybook clone:
  - _cargo generate --git https://github.com/daniman123/yew_storybook_clone.git_

## Usage

In order to run "Storybook" test suite, cd into dir "storybook_yew_app"

_cd storybook_yew_app_

Then run the command:

_trunk serve --open_

In order to run your Yew App, cd into dir "web_yew_app"

_cd web_yew_app_

Then run the command:

_trunk serve --open_
