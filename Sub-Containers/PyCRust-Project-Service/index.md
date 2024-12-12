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


### Quick setup PyCRust sub container
When you have previously installed this container, you can use the quick setup steps below. Otherwise please first read the [how to create a development container](https://nicojane.github.io/APP-X11-Forward-PyCRust-Development-Template-Stack/Howtos/howto_create_a_dev_container) document. <br>

1) **Create the WSLs**{: style="color:green; "} &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  <small>*(if not yet done*) </small>      
In case you don't have the **WSL** container, open CMD in the folder: *'APP-X11-Forward-PyCRust-Development-Template-Stack\Base-Container\Afx-Base-PyCRust-Service\'* and execute:
<pre class="nje-cmd-one-line"> wsl --import Ubuntu-docker-App-X11-Win32Dev ./wsl2-distro  "install.tar.gz"  </pre>

> *Remark:*{: style="color: black;font-size:13px; "} <br>
> <small>By default the WSL image is created in the sub folder of the current directory (./wsl2-distro) you may choose to store this image more **central**, for example like in 'd:\wsl-data\afx-stacks', this way you can **reuse** this WSL distribution for different **AFX stacks**  <br></small>


2) **Create docker base container (Afx-Base-PyCRust-Service)**{: style="color:green; "} &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;  <small>*(if not yet done*) </small> <br>
Create docker base container (in folder: 'Afx-Base-Service')
<pre class="nje-cmd-one-line">docker-compose -f  compose_app_forward_x11_pycrust_base.yml up -d --build --force-recreate  --remove-orphans </pre><br>

3) ***Create the sub container (PyCRust-Project-Service)***{: style="color:green; "} <br>
Create the Sub container (in folder: 'PyCRust-Project-Service'): 
<pre class="nje-cmd-one-line">docker-compose -f compose_pycrust_project.yml up -d --build --force-recreate --remove-orphans  </pre><br>
 
4) **Start the Docker sub container via the WSL**{: style="color:green; "} <small>*(optional*) </small> <br>
Execute the following commands: 
<pre class="nje-cmd-multi-line">wsl -d Ubuntu-docker-App-X11-Win32Dev 
docker exec -it PyCRust-Project-Service /bin/bash 
</pre>
> *Warning:*{: style="color: red;font-size:13px; "} <br>
> <small>When  the container cannot be found, restart the Docker app and ensure WSL integration is enabled in Docker settings! <br></small>

5) **Start the sub-container in Visual Studio Code**{: style="color:green; "}<br>
After this you should be able to open the container in VSC and start developing, be sure to run the following commands(in the container) first to make sure the required extensions are installed: 
<pre class="nje-cmd-multi-line">
code --install-extension ms-python.vscode-pylance
code --install-extension ms-python.python
code --install-extension ms-vscode.cpptools
code --install-extension ms-vscode.cmake-tools
code --install-extension ms-vscode.makefile-tools
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-vscode.cpptools-extension-pack
code --install-extension ms-python.debugpy
code --install-extension xyz.local-history
code --install-extension vadimcn.vscode-lldb
</pre>

<br><br>
