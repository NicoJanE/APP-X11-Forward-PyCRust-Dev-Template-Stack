---
layout: default_c
RefPages:
 - howto_create_a_dev_container   
--- 

<small>
<br><br>
_This file is part of: **Afx-X11-Forward-PyCRust-Template-Stack**_
_Copyright (c) 2024. Nico Jan Eelhart._
_This source code is licensed under the MIT License found in the  'LICENSE.md' file in the root directory of this source tree._
</small>
<br><br>3.2.1 Steps to Create a pycrust template application container

<div class="custom-style" style="--nje-color: #3077C8; --nje-size:12px; --nje-bg-color:#EDEDED">
**Quick links**{: style="color: #474747;font-size:12px; "} *(Reference to most common page sections)*{: style="color: #474747;font-size:11px; "}
- ***Basics***
  - [Install the WSL](#212-install-the-ubuntu-wsl-version)
  - [Configure the WSL](#213-configure-the-ubuntu-wsl-version)<br>
  - [Install the X-Server(client)](#214-install-the-x-server-vcxsrv)<br> 
  - [Create the base container](#215-create-the-basic-docker-container)<br> 
- ***Sub Containers***                      
  - [Steps to Create a pycrust template project container](#321-steps-to-create-a-pycrust-template-project-container)<br> 
  - [Attach to the PyCrust container](#322-attach-to-the-pycrust-container)<br> 
  - [Develop with VSC in the host](#4-develop-with-vsc-in-the-host)<br>
  - [Appendix 1 Quick Setup](#appendix-1-quick-setup) <br><br>
</div>  


<br>
# 1 What 
This is a Linux Docker cross-compiler image designed for applications with a Frontend module in Python and a Backend module in Rust (optionally combined with C/C++).

<br><br>
This Docker container (based on Ubuntu 24.04) is intended for use on a **Windows Docker Desktop host**. Its primary purpose is to facilitate the development and execution of a GUI application within a Linux Docker environment, while rendering the application's graphical interface on the Windows host. Since Docker itself is headless (i.e., it lacks built-in GUI support), additional steps are required to display the GUI application's output (for debugging or running in release mode). This functionality is achieved by integrating Docker Desktop with a **WSL2 environment** and the **VcXsrv X-11 server (XLaunch)**.

*Important*{: style="color: orange;font-size:13px; "} <small> This container is specifically designed for **Python** (frontend development) as well as **Rust** and **C/C++** (backend development). A similar stack exists for Win32 C development, called the ***App-X11-Forward-Win32 C Development Template Stack*** </small> <br>

The container stack is composed of multiple specialized containers, each built from its own Dockerfile and Compose file. At its core is the **Base container**, which provides essential build tools and dependencies. Note, however, that the Base container does not include any GUI frameworks or predefined project templates. 

Other containers in the stack, referred to as **sub-containers**, extend the Base container. Each sub-container incorporates specific frameworks and/or template projects, customized for particular development workflows or use cases.



### 1.1 Architecture of X11 forwarding
To help you better understand the container forwarding setup, the diagram below illustrates how the components interact after the setup is complete. The necessary components are installed on the Windows host (highlighted in the light blue square).

In addition to the processes shown in the diagram, this section also describes the folder structure and data file organization. These details are provided in the table titled Data Locations at the end of this section.

### The Base Container
![alt text](Context_arch.png)
The Windows Host runs a VcXsrv server **vcxsrv server** ([download](https://sourceforge.net/projects/vcxsrv/)) which handles the X-Protocol data sent from the Docker **Base Container**. This server uses the **WSL** environment as a back-end to process and prepare the data.

The Docker Base Container (grey square) is built on an Ubuntu image. It includes the Docker configuration files required to create itself, along with a Docker environment settings file named ***'.env'***

The **WSL environment** (yellow square) acts as the back-end for the Docker container. Although WSL2 operates using a lightweight virtual machine, it is specifically designed to run Linux distributions natively on Windows, enabling efficient and seamless access to Windows resources. The WSL environment processes graphical output from the Docker container, which is then sent to the **VcXsrv (XLaunch)** server on the host machine. To enable WSL2, Docker must be properly configured, and this configuration will be detailed later in this document.


The sub-containers represent specialized, self-contained environments designed to handle specific tasks or projects. For example, these may include projects written in Python, Rust, or C/C++. Each sub-container extends the Base Container with the necessary libraries and tools for its respective project. These sub-containers facilitate development workflows, including building, debugging, and running applications—such as **Win32** applications—using **Visual Studio Code (VSC)**.


 <sub>***Data Locations, Within root 'APP-X11-Forward-Win32-Template-Stack\'***</sub>

| **Folder**                                                                             | **Purpose**                                                             |
|:--------------------------------                                                       |:------------------------------------------------------------------------|
| <small>***./***</small>    	 				                             			                 | <small>***APP-X11-Forward-Win32-Development-Template-Stack***. The root folder |
| <small>***./Howto's***</small>   				 	                          			             | <small>Documentation</small> 											                                     |  
| <small>***./Base-Container***</small>    					                                     | <small>Root folder for the Base Containers</small>                                     |
| <small>***./Base-Container/Afx-Base-PyCRust-Service***</small>                         |<small>The Base Container .The docker and compose files</small>        |
| <small>***./Base-Container/Afx-Base-PyCRust-Service/wsl2distro***</small>              |<small>The WSL Windows environment after installation (.vhdx file) </small> |
| <small>***./Sub-Containers***</small>   					                                     |<small>Root folder for the Sub Containers</small>                                      |
| <small>***./Sub-Containers/PyCRust-Project-Service***</small>                          |<small>Sub Container with Win32 GUI C desktop program</small>        |
| <small>***./Sub-Containers/PyCRust-Project-Service/..devcontainer*** </small>          |<small>The settings of VSC when you open a container with VSC</small> |
| <small>***./Sub-Containers/PyCRust-Project-Service/.vscode*** </small>                 |<small>The settings of VSC when you open a container with VSC</small> |
| <small>***./Sub-Containers/PyCRust-Project-Service/Project-Template***</small>         |<small>Template used by Sub Container to create the project app</small>            |
| <small>***./Sub-Containers/PyCRust-Project-Service/shared-host***</small>              |<small>interaction from docker to host(i.e for backup) from </small>            |
| <small>***./Sub-Containers/PyCRust-Project-Service/.env***</small>                     |<small>Settings use by the Docker Compose and Dockerfile </small>                      |


<br>

<details closed>  
 <summary class="clickable-summary">
 <span  class="summary-icon"></span> 
 **Side note**: Security Considerations and Network Configuration
 </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->
  
>### Security Considerations and Network Configuration <br>
>For personal use on a developer's PC or laptop, security risks are generally lower than in a production environment. However, it's still a good idea to follow some basic security practices. When running an X server (like vcxsrv) on your Windows host, configure vcxsrv to only accept connections from the local machine. This minimizes exposure and ensures that only applications on your PC can access the server. Additionally, keep your firewall enabled and set to block unsolicited incoming connections. While this setup is for development purposes and may not require strict security measures, these simple steps can help protect your system against unexpected threats, especially when connected to less secure networks.
<br>
In practice, this means that as a developer, you should leave the XLaunch **'Extra option' -> Disable access control** ***unchecked***
</details>



# 2. Create the Base Container
This chapter will cover the setup of the **Base Container Service** (folder: 'Base-Container') and everything else required to enable X11 forwarding on a Windows host. At the end of this setup, we will demonstrate that the program works as expected by executing a sample X application in the Base Container (**xeyes**).  

## 2.1 The Basic Container Setup
Before executing the Docker Compose file, ensure that the following items are installed and configured (refer to section 1.1, Process Architecture). The steps for these items will be explained in more detail in the following paragraphs:

<span class="nje-ident" style="--nje-number-of-spaces: 15px;" ></span>  <small>**Overview**</small>
- **Download the  WSL version of Ubuntu**: Obtain the special version of Ubuntu for WSL   ([Download)](https://learn.microsoft.com/en-us/windows/wsl/install-manual). Scroll to the bottom of the page for manual versions.
- **Install WSL2**: Set up a dedicated WSL2 environment to serve as the backend for the Docker container.
- **Configure the WSL Ubuntu Distribution**: Ensure that the WSL Ubuntu distribution is properly configured.
- **Install and Configure an X-Server**: Install an X-server on the Windows host; we use VcXsrv  ([Download](https://sourceforge.net/projects/vcxsrv/)) for this purpose.
- **Run Docker to Create the Basic Image**: Execute the Docker files to create the basic container image.
- **Verify the Setup**: Display the result to demonstrate that everything works correctly.

> *Skip?*{: style="color: black; font-size:13px;"}
> <small>If you have previously installed other 'APP-X11-Forward-...' Docker containers, you likely already have an **Ubuntu-docker-App-X11-Win32Dev** WSL distribution. In that case, you can reuse the existing distribution and **skip** the remainder of this paragraph and **also** paragraph **2.1.2 till 2.1.3**.</small>



### 2.1.1 Download the Special Ubuntu WSL version
> *Remark:*{: style="color: black;font-size:13px; "}
> <small>If you have already installed the **WSL Ubuntu-Docker-App-X11-Win32Dev** WSL (as required by one of our other stacks), you don't need to do this again. You can reuse the already created WSL environment. In that case, you can proceed to: **2.1.4 Install the X-Server (VcXsrv)**.
<br></small>

Finding this version can be a bit challenging, especially because we need the manual installation files (with the .Appx or .AppxBundle extensions). The Windows Store provides a direct installer, but we cannot use it because we need to control the installation name and location. Follow these steps:
- ([Download](https://learn.microsoft.com/en-us/windows/wsl/install-manual)) the image from here, Scroll to almost the bottom where it states **'Downloading distributions'** and choose the *Ubuntu 24.04* link (note that this is the distribution  we support, you may try other ones and be fine with it, but we have not tested it)
- Now, as of Aug 2024, a lott of documentation\samples will state that your receive **\*.Appx** extension file and that you need to change the file to **\*.zip.**  But in our case you probably receive a **\*.AppxBundle** file which contains multiple Ubuntu versions. Below is shown how we get access to the right folder so we can install it in the next paragraph (in my case the download name is ***'Ubuntu2204-221101.AppxBundle'*** we use this name in our example:

  - For Ubuntu 22.04 LTS <small>(April 2027)</small>
    - First rename ***'Ubuntu2204-221101.AppxBundle'*** to ***'Ubuntu2204-221101.zip'***
    - Unpack the file with for example **7zip**
    - In the unpacked folder locate the file for your machine distribution ,likely ***'Ubuntu_2204.1.7.0_x64.appx'*** rename this file to *.zip and Unpack it
  - For Ubuntu 24.04 LTS  <small>(April 2029)</small>
    - First rename ***'Ubuntu2404-221101.AppxBundle'*** to ***'Ubuntu2404-221101.zip'***
    - Unpack the file with for example **7zip**
    - In the unpacked folder locate the file for your machine distribution ,likely ***'Ubuntu_2404.0.5.0_x64.appx'*** rename this file to *.zip and Unpack it
    
  In **both cases** in the last unpacked folder you should see a file called ***'install.tar.gz'*** this is the location where the next command has to point to.




### 2.1.2 Install the Ubuntu WSL version
When we have the distribution source, we can install the WSL environment. To keep the Base Container files in one place we do this in the root of our Base-Service folder ( ***'./Base-Container/Afx-BaseWin32-Service/wsl2distro'***).

> *TIP:*{: style="color: green; font-size: 15px;"} 
> <small>When you use **multiple** AFX Stacks, you can share the same WSL. These instructions install the WSL in a subfolder of the Stack (e.g., ./wsl2-distro). However, you can store the WSL in a central location. To do this, modify the command part **./wsl2-distro** to something like **D:/WSL/WSL-Data/Ubuntu-docker-App-X11-Win32Dev** <br> </small>

- **Open** in the sub folder: ***'.\Base-Container\Afx-Base-Win32-Service\'*** a CMD prompt.
- **Execute** this command and replace the ***"install.tar.gz.file"*** with the result from the previous step(full path)
<pre class="nje-cmd-one-line"> wsl --import Ubuntu-docker-App-X11-Win32Dev ./wsl2-distro  "install.tar.gz" </pre>
<span class="nje-ident"></span>This results in a **Ubuntu-docker-App-X11-Win32Dev** WSL in: **./wsl2-distro**. **Check**:
<pre class="nje-cmd-multi-line">

wsl --list --verbose    # Displays the distribution name, state, and there version
wsl --unregister YourDistributionName       # Remove the distribution
                                            # More WSL command in the next paragraph
</pre>

### 2.1.3 Configure the Ubuntu WSL version
To start and manage your WSL2 Ubuntu distribution, use the following command:
<pre class="nje-cmd-multi-line">

wsl -d Ubuntu-docker-App-X11-Win32Dev     #  This will open a CLI terminal and start the WSL if needed
                                          #  Use 'exit' to return to Windows. while it remains started
wsl --list --verbose                      #  Optional. Check if it is running (in other Windows CMD)
wsl --terminate Ubuntu-docker-App-X11-Win32Dev    #  Stops the distribution
wsl -d Ubuntu-docker-App-X11-Win32Dev -- ls /home #  Start, exec command, and returns direct(no CMD)
wsl --set-default Ubuntu-docker-App-X11-Win32Dev  #  Set default when running command; wsl

</pre>
Next we need to update and configure our distribution. Make sure our WSL distribution is started, and the execute the following Linux commands:
<pre class="nje-cmd-multi-line">

# 1.1                                   # Update the Ubuntu distribution
apt update && apt upgrade -y            

# 1.2.                                  # Make sure docker is installed in the WSL                    
apt update && apt install docker.io -y  # to be able to attach to a container in the Host 

# 2.1 The next command will update our DISPLAY environment variable
export DISPLAY=$(grep -oP "(?<=nameserver ).+" /etc/resolv.conf):0

# 2.2.                                  # Display the variable (check)
echo $DISPLAY                          

# 3.1 Make sure the Docker daemon is start at start up
echo -e "\n# Start Docker daemon if not running\nif (! pgrep -x \"dockerd\" > /dev/null); then\n    sudo  dockerd & \nfi" >> ~/.bashrc

# 3.2 Let make sure to easily identify the container (prompt)
echo 'PS1="\[\033[91m\]WSL:\[\033[0m\]\[\033[0;33m\]${debian_chroot:+($debian_chroot)}\u\[\033[0m\]:\[\033[91m\] App-X11-Win32Dev \[\033[0m\]../\W# "' >> ~/.bashrc

# 4 Make sure to reload the start-up command, to apply the 3.* commands
source ~/.bashrc                  # Reload
Enter                             # YOU need to give an extra enter so the prompt returns
                                  # check with: ps -a  should show dockerd

# Optional to logout and leave the wsl running
exit

</pre>

### 2.1.4 Install the X-Server (VcXsrv)
To install the X-server on the **Windows host** and receive graphical output from the application, follow these instructions (if not done yet):
- [Download]( https://sourceforge.net/projects/vcxsrv/) and Install the VcXsrv software.
- After installation start XLaunch
  - Select **Multiple Windows** and click **Next**
  - Select Start **no client* and click **Next**
  - Ensure that **Clipboard** and **Native opengl** are **enabled**'
  - Ensure that **Disable access control** is **not enabled** ( this is more secure; only enable it if you encounter issues) click **Next**, then **Finish**


### 2.1.5 Create the basic Docker Container
Let's get to the real thing and start creating the base container.
- Open the service sub folder: ***'.\Base-Container\Afx-Base-PyCRust-Service\\***' within a new CMD
- Make sure you are **login** into **Docker**
- We use a fixed IP address in the Compose file to make it easier to communicate with services, such as an SSH server (not used in this setup). While this is not strictly necessary, we have included it by default. If you encounter any issues, you may choose to remove it from the **compose_app_forward_x11_pycrust_base.yml** file. The pre-configured IP address used can be found in the **.env** file. see:
<pre class="nje-cmd-one-line-sm-ident"> FIXED_SUBNET  # Default: 172.25.0.0/16            FIXED_IP      # Default: 172.25.0.18</pre>


- Execute this command in the service sub folder
<pre class="nje-cmd-one-line-sm-ident"> docker-compose -f  compose_app_forward_x11_pycrust_base.yml up -d --build --force-recreate  --remove-orphans </pre>

> *Warning!*{: style="color: red;font-size:13px; "} <br>
> <small> When recreating the same container(service name) avoid subtle/annoying caching issues, to avoid irritation, make sure to:</small>
> - <small> delete the container</small>
> - <small> delete the volume and </small
> - <small> use the Docker prune command,so: </small>
> <pre class="nje-cmd-one-line-sm-ident"> docker system prune -a --volumes</pre>

<br>
**Result:**
- In Docker Desktop a container is present with the name:***afx-base-pycrust-service/axf-basic-pycrust-service-1***
- In Docker Desktop a image is present with the name:  ***eelhart/appforwardx11-pycrust-base:latest*** This image is used by other sub containers!

> *Note!*{: style="color: black;font-size:12px; "} <br>
> <small> The interesting parts will be inside the sub-containers that you are going to install next. For this reason, you can delete the created container, but you **must** keep the image file, as the sub-containers will need it!  </small>
>
>  <small>**Don't delete** the base container **yet!!**. First, let's ensure everything works as expected. This will be covered in the next subsections (2.1.6 and 2.1.7). </small>

<br>


### 2.1.6 Start Docker from the WSL Distribution
If you are running multiple WSL distributions, you cannot use Docker’s method to integrate additional WSL distributions, see **Side note: 'Assigning a WSL to Docker (When Using a Limited Number of WSLs)'**

This is because Docker may select any of the WSL distributions enabled via the **Docker setting**: 'Enable integration with additional distros:' (Settings, Resources, WSL integration). 
><small>*Important*{: style="color: red;"} In addition to opening the container from WSL as described below, it seems you still need to 'Enable the integration.' Please execute the steps in the **'side note'** and restart Docker Desktop! </small>

To ensure the intended WSL is used when opening the Docker container, whether from the command line or when attaching in Visual Studio Code, start it as follows:
- It is perhaps a good idea to **restart** the docker for desktop application (not just the container) on the host. Without this, you may running in trouble when starting the container from WSL (***Docker will complain: 'can find the container'*** )
- Ensure the container is started in Docker Desktop, so we can attach to it in WSL.
- Then execute these steps in the WL distribution:

<span class="nje-ident"></span> <small>(this assumes that you did not change the service name in the dockerfile or the WSL name in the previous steps)</small>

  <pre class="nje-cmd-multi-line"> 
  # To make sure the correct WSL is used by our Docker container:
  #
  # First check that our Docker container is running in Docker desktop host, so we can attach to it later in the WSL.
  
  # Start the correct WSL, docker inside the WSL will started also
  # so You may need to give a few returns after executing this command
  wsl -d Ubuntu-docker-App-X11-Win32Dev
  
  # Form within the WSL attach to our Docker container at the host:
  # if you changed the name use: 'docker ps' from the host to display the name
  docker exec -it afx-base-pycrust-service-axf-basic-pycrust-service-1 /bin/bash

    # Restart Docker application if the container can not be found!
    # And make sure the integration in Docker settings is set!
                   
  </pre> 
  <span class="nje-ident"></span> **After these command you can**: <br>
   <span class="nje-ident"></span> - Execute commands at the command line prompt (see: 2.1.7 Verify the Setup) <br>
  <span class="nje-ident"></span> -  Open the Docker container in **VSC**, knowing that correct WSL is assigned (see section 4)
  


<details closed>  
  <summary class="clickable-summary">
  <span  class="summary-icon"></span> 
  **Side note**: Assigning a WSL to Docker (When Using a Limited Number of WSLs)
  </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->  
>
> ##### Assigning a WSL to Docker (When Using a Limited Number of WSLs)
Since we are starting the Docker container directly from a running WSL distribution, the following procedure is unnecessary. You can use this procedure if you are not starting the container from within WSL and are only using a limited number of WSL distributions (or relying on the default WSL). However, when multiple WSL distributions are integrated with Docker, this method is less reliable because Docker may select any available distribution if no default is set. If a default WSL is configured, it will be used automatically
- To ensure that this WSL distribution is connected  to your Docker setup
    - In Docker -> Settings -> Resource -> WSL integration
    - In the **'Enable integration with additional distros:'** section (if you don't see this option,  press: Refetch distros)
    - Select ***Ubuntu-docker-App-X11*** **Make sure only this one is selected!**
    - Press Apply & Restart (You may need to restart the Docker container manually). **I had the experience that it did not do anything after pressing 'Apply', when Started Docker Desktop with Admin rights it was fine**
<br>
</details>



### 2.1.7 Verify the Setup
After running the command in 2.1.5 and 2.1.6 we can test if the setup **succeeded**. Make sure the docker container is started from our WSL (see 2.1.6 above)
##### Verify the X Output
- In th Docker container CLI prompt (which you just opened via the wsl) , started from the WSL,  enter:
<pre class="nje-cmd-one-line-sm-ident"> xeyes</pre> 
<span class="nje-ident"></span>**Expect result**: *This should display a pair of eyes in a Window (X is working properly)*.<br>
<span class="nje-ident"></span>*When you don't see it check if XLaunch is started on the host.*

#####  Verify the C/C++ Windows build environment (optional)
This section will test if the C++ GNU Windows cross-compilation is working, and that the 'wine' program can be used to output a Windows executable
  - Use the **'nano'** command to create a new file **'hello.c'** with this content:
  
  <pre class="nje-cmd-multi-line">
  #include &lt;windows.h&gt;

  int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    MessageBox(NULL, "Hello, Win32!", "Win32 Program", MB_OK);
    return 0;
  }
  </pre>
  - Check if we can build the program:
  <pre class="nje-cmd-multi-line">  BUILD: i686-w64-mingw32-gcc hello.c -o out.exe      # Creates 32 bits App
 #BUILD: x86_64-w64-mingw32-gcc hello.c -o out.exe    # Creates 64 bits App
 #BUILD: i686-w64-mingw32-c++ hello.c++ -o out.exe    # Creates 32 bits App
 #BUILD: x86_64-w64-mingw32-gcc hello.c++ -o out.exe  # Creates 64 bits App </pre>
  
  - And finally check if it runs
  <pre class="nje-cmd-multi-line">wine ./out.exe</pre>
<span class="nje-ident"></span>**Expect result**: *This should display a Window with 'Hello' in it (Build configured properly)*.<br>
<span class="nje-ident"></span>*Note that 'wine' is used to run the Windows executable, [more information here](https://www.winehq.org/)*


> *Warning!*{: style="color: red;font-size:13px; "} <br>
> <small> To use the Sub Containers, see next paragraph,  you may remove the container 'afx-win32-basic/axf-basic-win32-service-1' but you will need to hold on to the image **eelhart/appforwardx11-pycrust-base:latest**</small>

#####  Verify the Linux C/C++ build environment (optional)
Lets see if the default Linux gcc is available
<pre class="nje-cmd-multi-line">gcc -v
# This should display the Linux target in the string. It should contain:
# ...
# Target: x86_64-linux-gnu
# ...
# The following command returns all available targets:
ls /usr/bin/*gcc*
# Which should contain at least:
# ...
# /usr/bin/x86_64-linux-gnu-gcc-13
# /usr/bin/x86_64-w64-mingw32-gcc-13
# ...
</pre>

#####  Verify the Python and Rust environment (optional)
<pre class="nje-cmd-multi-line">python3 --version
# This should display the Python version, in my case : 3.12.3
# 
rustc --version
# This should display tte Rust version, in my case : 1.82.0
#
</pre>
When these tests are passed your good to go.



<details closed>  
  <summary class="clickable-summary">
  <span  class="summary-icon"></span> 
  **Side note**: Docker call syntax
  </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->  
>### Docker calling context <small> (***Skip this if you know Docker basics***) </small><br>
**Docker calling context**
Because we use Docker files (Dockerfile and compose) with descriptive names, such as **Dockerfile_Nodejs_React_Cont** instead of just **Dockerfile**, this affects how Docker commands are executed. For example, with a standard **Dockerfile**, we would use this command to reference  the Docker file in the **Docker Compose** file:
><pre class="nje-cmd-multi-line">
context: .
dockerfile: Dockefile	
></pre>
In our case, we cannot use the default name but have to specify the name we gave, thus:<br>
><pre class="nje-cmd-multi-line">
build: 	    
context: .
dockerfile: Dockerfile_Nodejs_React_Cont	    
></pre>
 The same applies for using the build command. With the default Dockerfile, you can use this:
 ><pre class="nje-cmd-multi-line">
docker build 
# This will assume a file: Dockerfile is available
></pre>
With the named file, we have to use
><pre class="nje-cmd-one-line">docker build -f MyDockerFileNameHere </pre> <br>
The same applies for running the Compose file (use **-f** option) 
<br>
</details>

<details closed>  
  <summary class="clickable-summary">
  <span  class="summary-icon"></span> 
  **Side note**: Create\customize Project from Template
  </summary> 	<!-- On same line is failure, Don't indent the following Markdown lines!  -->  
>### Create Project from Template
>>  <small> **Skip** this if you known how to deal with copy\customize docker files. Or if you just use the Github ***'Use this template'*** button </small> <br>
>
> To adapt the template directory for your project, follow these steps. This guide assumes you’re using the React stack; if you’re working with a different stack (e.g., PHP, Rust), simply replace “React” with the stack name your are using.
> - Copy the whole directory to a new directory (MyReactStack) for your project:
><pre class="nje-cmd-one-line"> copy "React Development Template Stack" MyReactStack </pre>
> - Within your **MyReactStack** open the ***[name]Service*** directory <br><br>
>**Warning**{: style="color: red;font-size:13px; "} <small>When using multiple containers, it's a good idea to rename this ***[name]Service*** directory (for example, by adding a number) before proceeding. Otherwise, the containers will be grouped together, which is generally helpful, but this can lead to caching issues in certain container stacks, such as React. These issues may manifest as the same directories appearing in the container from a previous instance after running the **compose_nodejs_react_cont.yml** command. Caching problems can be quite troublesome in some Docker stack configurations</small> 
>
> - Customize the Dockerfiles: Since most Docker Compose setups involve a parent-child relationship (i.e., chaining), a change in one Dockerfile requires updates to all related docker files.**Follow these steps:**
>> - In the first compose_\* file change the **'services name'** to an appropriate name for you:
>> <pre class="nje-cmd-one-line"> services: webserver-nodejs-react:	# Us always lowercase! </pre>
>> - The **'service name'** may appear more than once in the same file, update these as well!
>> - Changes the **'service name'** from step 1 in the other 'compose_\* files' 
>> - Check the compose_\* files when it contain a **image name** than update this to your own image name:
>><pre class="nje-cmd-multi-line">
context: .
	dockerfile: Dockerfile_Nodejs_React_Cont
	image: eelhart/react-base:latest  # i.e: [yourname/react-prjx]
>>
>></pre>
>  - Lastly, update the ports to ensure that each host port is unique across all running containers. In your Docker Compose file, you might see this configuration: <br>
><pre class="nje-cmd-multi-line">
ports:
  target: 3001        # Container port.
  published: 3002     # Host port, Make SURE it is unique	
>  
# Alternatively, the syntax might look like this (achieving the same result): 
ports:
  - "3002:3001"      # host:container  
>
></pre>
> **Make sure that Host port: 3002 is not used by any other docker container or other services on your host!**
<br><br>
</details>

## 2.2 What do we have and What's next?
We now have a Docker container that includes **Python**, **Rust**, and **GCC** for Linux, as well as **GCC** configured for use with **MinGW**. Additionally, the container provides the **Wine** emulator, allowing Windows binaries to be executed within the Linux environment. Any **GUI** output is **forwarded** to the host system using the **X11** protocol.

**MinGW** provides the **Win32 API**, enabling the creation of **Python Win32 desktop applications** for Windows. These applications can be executed directly within the Docker container using the **Wine** Windows emulator. The container's X11 configuration, together with the **XLaunch** server running on the Windows host, ensures that the executed application is displayed in a window on the Windows desktop host.

Next, proceed to **Section 3**, where you will set up a Docker sub-container containing a template application. This template **combines** a **Python** framework as the front-end with a **Rust/C/C++** implementation as the back-end.

<br>

# 3. Creating the Sub containers
This section explains how to create a **sub-container**, which houses the actual **pycrust** template project. Currently, there is only one sub-container, but additional template versions may be introduced in other sub-containers in the future. Before proceeding, ensure that you have already created the **Base Container** as described in **Section 2**, and verify that it is functioning correctly.

In the **Sub-Containers** folder, each sub-container is stored in its own separate folder. Sub-containers typically include specific project templates, such as Visual Studio Code settings, build tasks, and any required build tools or libraries. This particular image is designed for **Python**, **Rust**, and **C/C++** development, so the focus will be on these languages, their tools, and frameworks. The resulting application will be displayed in a window on the Windows host.

<br>
## 3.1 Creating pycrust template application <small>(compose_pycrust_project)</small>
This sub-container is used to create a project that combines a front-end (GUI/Web) with a back-end (Rust, C/C++) application. In the .env file, the following setting is used to install the template project: 
<pre class="nje-cmd-one-line">PRJ_TYPE_USE_CUSTOM_APP=Yes</pre> 
Refer to Section 4 for the global usage of this container in Visual Studio Code (**VSC**). Any specifics regarding the usage of this container in VSC will be detailed at the end of this section.


### 3.2.1 Steps to Create a pycrust template project container  
1. Open a Command Prompt in: ***.\Sub-Containers\PyCRust-Project-Service\\***
1. **Configure the project**:
  - Open the ***.env*** file to adjust the necessary settings: <br><br>
        - **Project Name**: Set the variable **PRJ_NAME_ARG** to your desired project name. This will be used for both the project name and the project directory.If omitted, the default value from **PRJ_NAME_ARG** in the **.env** file will be used.
        - **Network Configuration**: If needed, you can specify an alternative subnet and IP address by adjusting the variables **FIXED_SUBNET** and **FIXED_IP**.    
1. Execute the Docker command to create the project.:
<pre class="nje-cmd-one-line">docker-compose -f compose_pycrust_project.yml up -d --build --force-recreate --remove-orphans  </pre>


### 3.2.2 Attach to the pycrust container
After running the commands in 3.1 you can start the **pycrust sub container'** in combination with the WSL. See the **side note: Start Docker via WSL** if you need help with this.  Once started you can use Visual Studio Code (VSC) to start developing the application you planned to, for help with VSC see Section 4.  Here in short the steps to start/attach to the container in a WSL distribution
- In a OS Terminal: Start the WSL and start Docker in the WSL:
<pre class="nje-cmd-multi-line">

wsl -d Ubuntu-docker-App-X11-Win32Dev 

# In the Resulting WSL terminal attach the docker container:
#   - When it is not started you can do this here with:
      docker exec -it pycrust-project-service-axf-pycrust-project-1  /bin/bash
#     
# In case of err see 'Error checks'

# Use command 'pwd' to check your directory location
# Make sure XLaunch is started on the host!

</pre>>

 <span class="nje-ident"></span> *Error checks:*{: style="color: red;font-size:12px; "} <small>When the container can't be found, first try to restart the Docker application! Also consider:
 <br><span class="nje-ident" style="--nje-number-of-spaces: 120px;"></span> * Check if the WSL is integrated in Docker settings
 <br><span class="nje-ident" style="--nje-number-of-spaces: 120px;"></span> * Check if the docker demon is started in the WSL (ps -aux -->  sudo dockerd)
 </small> 

Commands to check your installation in the Docker terminal:
<pre class="nje-cmd-multi-line">

pwd                       # Should displays your project directory including source code 
xeyes                     # Displays gazing eyes
 </pre>
<br>
Now You can develop in Visual Studio Code, for help and additional instructions see 4

# 4 Develop with VSC in the host
To develop in **V**isual **S**tudio **C**ode we advice the following instructions 

First make sure you have the following extensions locally install, to be able to work with the docker containers:
- **Remote Development**. 
  Attach VSC to Docker(Develop directly inside our Docker container ), Remote WSL, Develope on remote SSH, GitHub Codespaces. 
- **Docker** (Microsoft)
- **Dev Containers** (Microsoft) optional


### 4.1. Open the C++ application container in VSC (@host)
- Mak sure Docker can be attached from the WSL! See [here](#322-attach-to-the-pycrust-container)
- Press CTRL-SHIFT-P or F1 and select (start typing) **Attach to running container...**
- Select our **pycrust-project-service-axf-pycrust-project-1** container
- Alternatively you might click on the **Docker boot** on the left toolbar and select the container from there.  
This opens a new Window with the  container information

### 4.2. Open Folder and building your app.
***This should not be needed cause the container should start in the created project directory automatic***
- Use the **VSC Explorer** and the **Open Folder** to open the remote container's folder. **Ensure** you open the correct folder so that the **.vscode** directory settings are applied properly.
- Select Open Folder and enter: **/projects/pycrust/project_name**. This will ensure the project is loaded along with the settings configured in the .vscode folder. (Alternatively, you can obtain the path by opening a terminal inside the Docker container. The initial folder shown by the pwd command will give you the correct path.)

> *Notice: Recommend extensions*{: style="color: gray;font-size:13px; "}<br>
> <small>For this Docker project we have a few extensions defined, please allow these in during the opening of the container, so when you see something like: ***'Do you want to install the recommended extensions from ...'*** Press on the **Install**  button, this makes sure all functionality will work as defined, see the file ***.vscode/extensions.json*** for the recommend extension list.    <br></small>

When opening the **pycrust** container and the project root folder in Visual Studio Code, a dedicated Visual Studio Code server will be installed within the container. This server provides a full Visual Studio Code environment with its own settings and extensions, which we have provided (see the side note below). Upon opening the folder for the first time, the system will detect any required extensions and may prompt you to install them. If so, follow the instructions to complete the installation. For a list of extensions, refer to the side note below.


## 4.3 Visual Studio Code container extensions
Specific extensions for this container are installed. Local extensions should be disabled for this container to prevent poor performance in Visual Studio Code and avoid other side effects. Container-specific extensions are listed in the file located at: ***.devcontainer/devcontainer.json.***
and are required, included extensions are: 

<div class="custom-style" style="--nje-color: #8b5e3c; --nje-size: 12px;">
<span class="nje-ident"></span> ms-vscode.cpptools <br>
<span class="nje-ident"></span> ms-vscode.cmake-tools <br>
<span class="nje-ident"></span> ms-vscode.cpptools-extension-pack<br>
<span class="nje-ident"></span> ms-vscode.makefile-tools<br>
<span class="nje-ident"></span> ms-python.vscode-pylance<br>
<span class="nje-ident"></span> ms-python.python<br>
<span class="nje-ident"></span> "ms-python.debugpy"<br>
<span class="nje-ident"></span> "vadimcn.vscode-lldb"<br>
<span class="nje-ident"></span> rust-lang.rust-analyzer"<br>
<span class="nje-ident"></span> xyz.local-history<br>
</div>

> *Warning:*{: style="color: red;font-size:13px; "}
> <small>While these extensions should install automatically, I have experience issues with them. Check is they if are installed, and if not them manually with the command below:  <br></small>
In the opened container oin the terminal session enter:
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
Similar to the extensions, we also provide default settings for this container. These settings define the global configurations as well as the default build and run behavior. The build and run behavior is explained in the next section (4.3). See the side note below for details.
<br>



### 4.3 VSC Build tasks
In the menu **'Terminal -> Run Tasks...'** You can find the build task for our project, which are defined in the .vscode directory in the file 'tasks.json'. All our build task start with the prefix: AFX (_AFX are mplementation task and are by default hidden)
| **Task**                                                   | ***Description**                                                               |
|:------------------------------                             |:-----------------------------------------------                                |
| AFX BUILD 1: Libraries (Rust,LINUX)                        | Builds the Rust libraries inside: src_backend\api_interface_rust for Linux. User get question which build             |
| AFX BUILD 2: Libraries (Rust,WINDOWS)                      | Builds the Rust libraries inside: src_backend\api_interface_rust for Windows. User get question which build |
| AFX BUILD 3: Client Library Tester (Rust LINUX, debug)     | Builds the client test program related to the Rust library in  1(see also create 5).|
| AFX Create 4: business Library (Rust)                      | This create a whole ne Rust library in: src_backend\api_interface_rust |
| AFX Create 5: Client Library Tester (Rust LINUX)           | This Creates the client test program for a Rust library (see also 3 for build)|
| AFX CLEAN 6: Rust Libraries (LINUX Debug,release)          | Removes all build artifacts for the Linux release, Debug and Release|
| AFX CLEAN 7: Rust Libraries (WINDOWS Debug,release)        | Removes all build artifacts for the Windos release, Debug and Release||

### 4.4 Run &Debug
A few launches are defined
- **Python (Debugpy)**  for python
- **Python + Rust** for Python and Rust libraries

In All case you must make sure tha the Rust library is build AFX BUILD task 1. This is beacuse the current main.py sample call the Rust (Rust call internal and other library which is build together with Rust) and functions



### 4.5  Backup
There is a simple backup script which you can use to backup the project to **shared-host** directory. In a Docker shell (root) execute:
<pre class="nje-cmd-one-line">./_backup </pre>



<br><br><br>

## Appendix 1. Quick setup
If you have previously installed this container, you can use the quick setup steps below. Otherwise please first read the [how to create a development container](https://nicojane.github.io/APP-X11-Forward-PyCRust-Development-Template-Stack/Howtos/howto_create_a_dev_container) document.
- In case you don't have the **WSL** container, open CMD in the folder: ***'APP-X11-Forward-PyCRust-Development-Template-Stack\Base-Container\Afx-Base-PyCRust-Service\'*** and execute:
<pre class="nje-cmd-one-line"> wsl --import Ubuntu-docker-App-X11-Win32Dev ./wsl2-distro  "install.tar.gz"  </pre>
- Create docker base container (Afx-Base-PyCRust-Service)
 <pre class="nje-cmd-one-line">docker-compose -f  compose_app_forward_x11_pycrust_base.yml up -d --build --force-recreate  --remove-orphans </pre>
 - Install C++ sub-container (PyCRust-Project-Service)
  <pre class="nje-cmd-one-line">docker-compose -f compose_pycrust_project.yml up -d --build --force-recreate --remove-orphans  </pre>
  - Attach docker to the WSL
  <pre class="nje-cmd-multi-line">
# Start WSL
wsl -d Ubuntu-docker-App-X11-Win32Dev  

# Attach docker
docker exec -it PyCRust-Project-Service /bin/bash 
# If the container cannot be found, restart the Docker app and ensure 
# WSL integration is enabled in Docker settings!
</pre>

# VSC extensions
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


<br><br><br>

## Appendix 2. pycrust template structure

Following is a concept diagram of the sample Rust library 'core_lib' which internally opens an other C++ library and call a function from there

![alt text](Main_v2_snip.jpg)