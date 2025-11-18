---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 


# Python, Rust, C++ <span style="color: #409EFF; font-size: 0.6em; font-style: italic;"> -  Docker Container</span>


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
        <a href="./Sub-Containers/PyCRust-Project-Service/index">Setup Guide</a>
    </span>
</span>
 </div>

<span style="color: #6d757dff; font-size: 10px; font-style: italic;"> <br>
This file is part of: **Afx-X11-Forward-PyCRust-Template-Stack**
Copyright (c) 2025 Nico Jan Eelhart. This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree.</span>

<p align="center">─── ✦ ───</p>
