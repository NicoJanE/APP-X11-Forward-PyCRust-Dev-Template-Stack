---
layout: default_c
RefPages:
 - howto_create_a_dev_container
--- 

<small>
<br>
_This file is part of: **Afx-X11-Forward-PyCRust-Template-Stack**_
_Copyright (c) 2024 Nico Jan Eelhart_
_This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree._
</small>
<br><br>

## What
A Docker Template Stack Container designed for creating a frontend application in Python, paired with a backend service built using Rust and/or C/C++.

The stack allows you to build both a native Linux version and a cross-compiled Win32 version using MinGW, with support for Visual Studio Code and related tools.

Since this template uses an Afx-X11-Forward approach, any desktop GUI developed within the container will be forwarded to the Windows host via the X11 Protocol, using tools like XLaunch on the host.
 

#### Available Sub-container

| ***Container name***  | ***Quick reference (if available)*** |
|:-----------------     |:----------------|
| axf-pycrust-project   | [Specific quick setup](Sub-Containers\PyCRust-Project-Service\index)  |


### Quick setup (general)
This is the general quick setup procedure. In the sub-container project you may find index.md file with specific quick setup instruction in case these instruction make you feel insecure.

If you have previously installed this container, you can use these general quick setup steps below. Otherwise please first read the [how to create a development container](https://nicojane.github.io/APP-X11-Forward-PyCRust-Dev-Template-Stack//Howtos/howto_create_a_dev_container) document. or refer to one of the 'index.md' files in 

the sub-container, when available it contains detailed instructions for the specific sub-container  

1) **Create the WSLs**{: style="color:green; "} <br>
In case you don't have the **WSL** container, open CMD in a base container folder, for example: *'APP-X11-Forward-PyCRust-Development-Template-Stack\Base-Container\Afx-Base-PyCRust-Service\'* and execute:
<pre class="nje-cmd-one-line"> wsl --import Ubuntu-docker-App-X11-Win32Dev ./wsl2-distro  "install.tar.gz"  </pre>

> *Remark:*{: style="color: black;font-size:13px; "} <br>
> <small>By default the WSL image is created in the sub folder of the current directory (./wsl2-distro) you may choose to store this image more **central**, for example like in 'd:\wsl-data\afx-stacks', this way you can **reuse** this WSL distribution for different **AFX stacks**  <br></small>

2) **Create docker base container (Afx-Base-PyCRust-Service)**{: style="color:green; "} <br>
Use the following to create the basic docker base container, on which the sub-containers depend
 <pre class="nje-cmd-one-line">docker-compose -f  compose_app_forward_x11_pycrust_base.yml up -d --build --force-recreate  --remove-orphans </pre><br>
3) **Create a specific  docker sub-container**{: style="color:green; "} <br>
 Create a Command Line Shell in the correct sub-container folder (i.e. PyCRust-Project-Service) and execute:
  <pre class="nje-cmd-one-line">docker-compose -f compose_pycrust_project.yml up -d --build --force-recreate --remove-orphans  </pre> <br>
4) **Start the Docker sub container via the WSL**{: style="color:green; "} <br>
Execute the following commands: 
<pre class="nje-cmd-multi-line">wsl -d Ubuntu-docker-App-X11-Win32Dev 
docker exec -it PyCRust-Project-Service /bin/bash   # i.e for PyCRust Sub container
</pre>
> *Warning:*{: style="color: red;font-size:13px; "} <br>
> <small>When  the container cannot be found, restart the Docker app and ensure WSL integration is enabled in Docker settings! <br></small> <br>

5) **Configure and start the sub container in  Visual Studio Code (VSC)**{: style="color:green; "}<br>
After this you should be able to open the container in VSC and start developing, be sure to run the required extension commands(in the container)    


