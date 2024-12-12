---
layout: default_c
RefPages:
 - howto_create_a_dev_container   
--- 


# Hierarchy of Docker Services
A Docker sub-container can extend or depend on at most one other service, which may either be the base service or another sub-container. The table below shows the service each sub-container depends on. These dependent services (the ones it extends from) must be installed before the sub-container to be installed.

Note that the service name refers to the name in the Docker Compose file, which is part of the name you see in Docker (Desktop).


| **Service**                       | Extends                   |<small> Remark</small>
|:----------------------------------|:-------                   |:-----              
|***`axf-basic-pycrust-service`***  |None                       |<small>Service may be deleted, image **must** exist!</small>
|***`axf-pycrust-project`***        |axf-basic-pycrust-service  |<small>Python, Rust, C/C++ template project(VSC+MinGW+Wine) </small>
