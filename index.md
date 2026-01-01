---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 


# Python/Rust/C++ Development Stack <span style="color: #409EFF; font-size: 0.6em; font-style: italic;"> -  Docker Container</span>

![MIT License](https://img.shields.io/badge/License-MIT-green) ![Commercial Services Available](https://img.shields.io/badge/Services-Optional-blue)

## ℹ️ Introduction

A Docker Template Stack Container designed for creating a frontend application in Python, paired with a backend service built using Rust and/or C/C++. The stack allows you to build both a native Linux version and a cross-compiled Win32 version using MinGW, with support for Visual Studio Code and related tools.

Since this template uses an Afx-X11-Forward approach, any desktop GUI developed within the container will be forwarded to the Windows host via the X11 Protocol, using tools like XLaunch on the host.

## Setup

Below is an overview of the available containers and the documentation to setup the Containers, for the quick setup see the **Quick Setup**

### Required Base Container Setup

For the full documentation see the Setup Guide of the Base Container, which is followed by documentation about the sub containers listed below

<div class="nje-table-base-span">
<span class="nje-table-row">
        <span class="nje-column1-value">Required Base Container </span>
        <span class="nje-column2-desc">Core X11 forwarding infrastructure and GUI display capabilities</span>
        <span class="nje-column3-button"> 
            <a href="https://nicojane.github.io/APP-X11-Forward-PyCRust-Dev-Template-Stack/Howtos/howto_create_a_dev_container.html">Full Setup Guide</a>
        </span>
 </span>
 </div>

#### Sub Container Setup

In addition to the base container, which is required, the actual Win32 C project template is contained in a sub container. Currently we have these sub containers:

<div class="nje-table-sub-span">
<span class="nje-table-row">
    <span class="nje-column1-value">axf-pycrust-project</span>
    <span class="nje-column2-desc">.Python/Rust/C++ development environment and template </span>
    <span class="nje-column3-button"> 
        <a href="https://nicojane.github.io/APP-X11-Forward-PyCRust-Dev-Template-Stack/Howtos/howto_create_a_dev_container.html#appendix-1-quick-setup">Setup Guide</a>
    </span>
</span>
 </div>
<br>

<span style="color: #6d757dff; font-size: 13px; font-style: italic;"> 
<i><b>License</b><br>This file is part of: **Python/Rust/C++ Development Stack**  Copyright (c) 2025-2026 Nico Jan Eelhart.This repository is [MIT licensed](MIT-license.md) and free to use. For optional commercial support, customization, training, or long-term maintenance, see [COMMERCIAL.md](COMMERCIAL.md).</i>
</span>

<br><br>
<p align="center">─── ✦ ───</p>
